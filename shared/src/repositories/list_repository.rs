use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::{
    article_model::FullArticle,
    enums::Visibility,
    list_model::{CreateList, List, UpdateList},
    series_model::Series,
    tag_model::Tag,
    user_model::FullUser,
};

#[async_trait]
pub trait ListRepository<DB, E>
where
    DB: Database,
{
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        query: Option<&str>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<List>, E>;
    async fn find(
        transaction: &mut Transaction<'_, DB>,
        list_id: &str,
        by_user: Option<&str>,
    ) -> Result<List, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        create_list: &CreateList,
    ) -> Result<List, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_list: &UpdateList,
    ) -> Result<List, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, list_id: &str) -> Result<List, E>;
    async fn find_articles(
        transaction: &mut Transaction<'_, DB>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
        list_id: &str,
    ) -> Result<Vec<FullArticle>, E>;
    async fn add_article(
        transaction: &mut Transaction<'_, DB>,
        list_id: &str,
        article_id: &str,
    ) -> Result<(String, String), E>;
    async fn remove_article(
        transaction: &mut Transaction<'_, DB>,
        list_id: &str,
        article_id: &str,
    ) -> Result<(String, String), E>;
}

#[derive(Debug, Clone)]
pub struct ListRepositoryImpl;

#[async_trait]
impl ListRepository<Postgres, Error> for ListRepositoryImpl {
    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        query: Option<&str>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<List>, Error> {
        sqlx::query_as!(
            List,
            r#"
            SELECT
                id,
                user_id,
                label,
                image,
                visibility AS "visibility: Visibility",
                article_count,
                created_at,
                updated_at
            FROM lists
            WHERE (($2::timestamptz IS NULL AND $3::text IS NULL)
                    OR (created_at, id) < ($2, $3))
                AND visibility = 'PUBLIC' OR
                      $4 = user_id AND visibility in ('BYLINK', 'PRIVATE')
                AND ($5::text IS NULL OR label ILIKE $5)
            ORDER BY created_at DESC, id DESC 
            LIMIT $1
            "#,
            limit,
            created_at,
            id,
            by_user,
            query.map(|q| format!("%{}%", q))
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        list_id: &str,
        by_user: Option<&str>,
    ) -> Result<List, Error> {
        sqlx::query_as!(
            List,
            r#"
            SELECT
                id,
                user_id,
                label,
                image,
                visibility AS "visibility: Visibility",
                article_count,
                created_at,
                updated_at
            FROM lists
            WHERE id = $1
                AND (($2 = user_id AND visibility = 'PRIVATE')
                    OR visibility in ('PUBLIC', 'BYLINK'))
            ORDER BY created_at DESC
            "#n,
            list_id,
            by_user
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        create_list: &CreateList,
    ) -> Result<List, Error> {
        sqlx::query_as!(
            List,
            r#"
            INSERT INTO Lists (
                user_id,
                label,
                image,
                visibility
                )
            VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                user_id,
                label,
                image,
                visibility AS "visibility: Visibility",
                article_count,
                created_at,
                updated_at
            "#n,
            create_list.user_id,
            create_list.label,
            create_list.image,
            create_list.visibility as Visibility
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        update_list: &UpdateList,
    ) -> Result<List, Error> {
        sqlx::query_as!(
            List,
            r#"
            UPDATE lists
            SET label = COALESCE($2, lists.label),
                image = COALESCE($3, lists.image),
                visibility = COALESCE($4, lists.visibility)
            WHERE id = $1
            RETURNING
                id,
                user_id,
                label,
                image,
                visibility AS "visibility: Visibility",
                article_count,
                created_at,
                updated_at
            "#n,
            update_list.id,
            update_list.label,
            update_list.image,
            update_list.visibility as Option<Visibility>
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        list_id: &str,
    ) -> Result<List, Error> {
        sqlx::query_as!(
            List,
            r#"
            DELETE FROM lists
            WHERE id = $1
            RETURNING
                id,
                user_id,
                label,
                image,
                visibility AS "visibility: Visibility",
                article_count,
                created_at,
                updated_at
            "#n,
            list_id,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_articles(
        transaction: &mut Transaction<'_, Postgres>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
        list_id: &str,
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
                        WHEN $4 IS NULL THEN FALSE
                        WHEN f.follower_id IS NOT NULL THEN TRUE
                        ELSE FALSE
                    END AS followed
                FROM users u
                LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $4
            )
            SELECT
                a.*,
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT f.*) FILTER (WHERE f.id IS NOT NULL), NULL) as "users: Vec<FullUser>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT t.*) FILTER (WHERE t.slug IS NOT NULL), NULL) as "tags: Vec<Tag>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT s.*) FILTER (WHERE s.id IS NOT NULL), null) as "series: Vec<Series>",
                CASE
                    WHEN $4::text is NULL then ARRAY[]::lists[]
                    ELSE ARRAY_REMOVE(ARRAY_AGG(DISTINCT l.*) FILTER (WHERE l.id IS NOT NULL), NULL)
                END AS "lists: Vec<List>",
                CASE
                    WHEN $4::text IS NULL THEN FALSE
                    WHEN li.user_id IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS liked,
                sa.order AS "order: Option<f32>"
            FROM articles a
            LEFT JOIN authors au ON a.id = au.article_id
            LEFT JOIN followers f ON au.author_id = f.id
            LEFT JOIN likes li ON a.id = li.article_id AND li.user_id = $5
            LEFT JOIN articletags at ON a.id = at.article_id
            LEFT JOIN tags t ON at.tag_slug = t.slug
            LEFT JOIN listarticle la ON a.id = la.article_id
            LEFT JOIN lists l ON la.list_id = l.id AND l.user_id = $5
            LEFT JOIN seriesarticle sa ON a.id = sa.article_id
            LEFT JOIN series s ON sa.series_id = s.id
            WHERE l.id = $5 AND (($2::TEXT IS NULL AND $3::TIMESTAMPTZ IS NULL) OR (a.created_at, a.id) < ($3, $2))
            GROUP BY a.id, s.id, li.user_id, sa.order
            ORDER BY a.created_at DESC, a.id DESC
            LIMIT $1
            "#n,
            limit,
            id,
            created_at,
            by_user,
            list_id,
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn add_article(
        transaction: &mut Transaction<'_, Postgres>,
        list_id: &str,
        article_id: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            WITH list AS (
                UPDATE lists
                SET
                    article_count = article_count + 1
                WHERE id = $2
                RETURNING *
            )
            INSERT INTO listarticle (article_id, list_id)
            VALUES ($1, (SELECT id FROM list))
            "#n,
            article_id,
            list_id
        )
        .execute(&mut **transaction)
        .await;
        Ok((list_id.to_string(), article_id.to_string()))
    }

    async fn remove_article(
        transaction: &mut Transaction<'_, Postgres>,
        list_id: &str,
        article_id: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            WITH list AS (
                UPDATE lists
                SET
                    article_count = article_count - 1
                WHERE id = $1
                RETURNING *
            )
            DELETE FROM listarticle
            WHERE list_id = (SELECT id FROM list) AND article_id = $2
            "#n,
            list_id,
            article_id
        )
        .fetch_one(&mut **transaction)
        .await;
        Ok((list_id.to_string(), article_id.to_string()))
    }
}
