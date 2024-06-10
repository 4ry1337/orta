use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::{
    article_model::FullArticle,
    list_model::List,
    series_model::{CreateSeries, Series, UpdateSeries},
    tag_model::Tag,
    user_model::FullUser,
};

#[async_trait]
pub trait SeriesRepository<DB, E>
where
    DB: Database,
{
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        query: Option<&str>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Vec<Series>, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        create_series: &CreateSeries,
    ) -> Result<Series, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_series: &UpdateSeries,
    ) -> Result<Series, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, series_id: &str) -> Result<Series, E>;
    async fn find(transaction: &mut Transaction<'_, DB>, series_id: &str) -> Result<Series, E>;
    async fn find_articles(
        transaction: &mut Transaction<'_, DB>,
        limit: i64,
        order: Option<f32>,
        by_user: Option<&str>,
        series_id: &str,
    ) -> Result<Vec<FullArticle>, E>;
    async fn add_article(
        transaction: &mut Transaction<'_, DB>,
        series_id: &str,
        article_id: &str,
    ) -> Result<(String, String), E>;
    async fn reorder_article(
        transaction: &mut Transaction<'_, DB>,
        series_id: &str,
        article_id: &str,
        new_order: f32,
    ) -> Result<(String, String), E>;
    async fn remove_article(
        transaction: &mut Transaction<'_, DB>,
        series_id: &str,
        article_id: &str,
    ) -> Result<(String, String), E>;
}

#[derive(Debug, Clone)]
pub struct SeriesRepositoryImpl;

#[async_trait]
impl SeriesRepository<Postgres, Error> for SeriesRepositoryImpl {
    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        query: Option<&str>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Vec<Series>, Error> {
        sqlx::query_as!(
            Series,
            r#"
            SELECT *
            FROM series
            WHERE (($2::timestamptz IS NULL AND $3::text IS NULL)
                    OR (created_at, id) < ($2, $3))
                    AND ($4::text IS NULL OR label ILIKE $4) 
            ORDER BY created_at DESC, id DESC 
            LIMIT $1
            "#,
            limit,
            created_at,
            id,
            query.map(|q| format!("%{}%", q))
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: &str,
    ) -> Result<Series, Error> {
        sqlx::query_as!(
            Series,
            r#"
            SELECT *
            FROM series
            WHERE id = $1
            "#n,
            series_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        create_series: &CreateSeries,
    ) -> Result<Series, Error> {
        sqlx::query_as!(
            Series,
            r#"
            INSERT INTO series (
                user_id,
                label,
                image
            )
            VALUES ($1, $2, $3)
            RETURNING *
            "#n,
            create_series.user_id,
            create_series.label,
            create_series.image
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        update_series: &UpdateSeries,
    ) -> Result<Series, Error> {
        sqlx::query_as!(
            Series,
            r#"
            UPDATE series
            SET 
                label = coalesce($2, series.label),
                image = coalesce($3, series.image)
            WHERE id = $1
            RETURNING *
            "#n,
            update_series.id,
            update_series.label,
            update_series.image,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: &str,
    ) -> Result<Series, Error> {
        sqlx::query_as!(
            Series,
            r#"
            DELETE FROM series
            WHERE id = $1
            RETURNING *
            "#n,
            series_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_articles(
        transaction: &mut Transaction<'_, Postgres>,
        limit: i64,
        order: Option<f32>,
        by_user: Option<&str>,
        series_id: &str,
    ) -> Result<Vec<FullArticle>, Error> {
        sqlx::query_as!(
            FullArticle,
            r#"
            WITH followers AS (
                SELECT
                    u.id,
                    u.username,
                    u.email,
                    u.email_verified,
                    u.image,
                    u.bio,
                    u.urls,
                    u.follower_count,
                    u.following_count,
                    u.created_at,
                    u.approved_at,
                    u.deleted_at,
                    CASE
                        WHEN $3 IS NULL THEN FALSE
                        WHEN f.follower_id IS NOT NULL THEN TRUE
                        ELSE FALSE
                    END AS followed
                FROM users u
                LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $3
            )
            SELECT
                a.*,
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT f.*) FILTER (WHERE f.id IS NOT NULL), NULL) as "users: Vec<FullUser>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT t.*) FILTER (WHERE t.slug IS NOT NULL), NULL) as "tags: Vec<Tag>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT s.*) FILTER (WHERE s.id IS NOT NULL), null) as "series: Vec<Series>",
                CASE
                    WHEN $3::text is NULL then ARRAY[]::lists[]
                    ELSE ARRAY_REMOVE(ARRAY_AGG(DISTINCT l.*) FILTER (WHERE l.id IS NOT NULL), NULL)
                END AS "lists: Vec<List>",
                CASE
                    WHEN $3::text IS NULL THEN FALSE
                    WHEN li.user_id IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS liked,
                sa.order AS "order: Option<f32>"
            FROM articles a
            LEFT JOIN authors au ON a.id = au.article_id
            LEFT JOIN followers f ON au.author_id = f.id
            LEFT JOIN likes li ON a.id = li.article_id AND li.user_id = $3
            LEFT JOIN articletags at ON a.id = at.article_id
            LEFT JOIN tags t ON at.tag_slug = t.slug
            LEFT JOIN listarticle la ON a.id = la.article_id
            LEFT JOIN lists l ON la.list_id = l.id AND l.user_id = $3
            LEFT JOIN seriesarticle sa ON a.id = sa.article_id
            LEFT JOIN series s ON sa.series_id = s.id
            WHERE sa.series_id = $4 AND ($2::REAL IS NULL OR sa.order > $2)
            GROUP BY a.id, li.user_id, sa.order
            ORDER BY sa.order ASC
            LIMIT $1
            "#n,
            limit,
            order,
            by_user,
            series_id,
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn add_article(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: &str,
        article_id: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            WITH s AS (
                UPDATE series
                SET
                    article_count = article_count + 1
                WHERE id = $1
                RETURNING *
            )
            INSERT INTO seriesarticle (series_id, article_id, "order")
            VALUES ((SELECT id FROM s), $2, COALESCE(((
                SELECT max("order")
                FROM seriesarticle
                WHERE series_id = $1
            ) + 100), 0))
            "#n,
            series_id,
            article_id,
        )
        .execute(&mut **transaction)
        .await?;

        Ok((series_id.to_string(), article_id.to_string()))
    }

    async fn reorder_article(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: &str,
        article_id: &str,
        new_order: f32,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            UPDATE seriesarticle
            SET "order" = $3
            WHERE series_id = $1 AND article_id = $2
            "#n,
            series_id,
            article_id,
            new_order
        )
        .execute(&mut **transaction)
        .await;

        Ok((series_id.to_string(), article_id.to_string()))
    }

    async fn remove_article(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: &str,
        article_id: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            WITH s AS (
                UPDATE series
                SET
                    article_count = article_count - 1
                WHERE id = $1
                RETURNING *
            )
            DELETE FROM seriesarticle
            WHERE series_id = (SELECT id FROM s) AND article_id = $2
            "#n,
            series_id,
            article_id
        )
        .execute(&mut **transaction)
        .await?;

        Ok((series_id.to_string(), article_id.to_string()))
    }
}
