use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::series_model::{CreateSeries, Series, UpdateSeries};

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
        user_id: Option<&str>,
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
        _query: Option<&str>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        user_id: Option<&str>,
    ) -> Result<Vec<Series>, Error> {
        sqlx::query_as!(
            Series,
            r#"
            SELECT *
            FROM series
            WHERE user_id = COALESCE($4, user_id)
                AND (($2::timestamptz IS NULL AND $3::text IS NULL)
                    OR (created_at, id) < ($2, $3))
            ORDER BY created_at DESC, id DESC 
            LIMIT $1
            "#,
            limit,
            created_at,
            id,
            user_id,
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
