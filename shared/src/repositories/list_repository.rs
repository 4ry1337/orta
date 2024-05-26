use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::{
    enums::Visibility,
    list_model::{CreateList, List, UpdateList},
};

#[async_trait]
pub trait ListRepository<DB, E>
where
    DB: Database,
{
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        user_id: Option<&str>,
    ) -> Result<Vec<List>, E>;
    async fn find(transaction: &mut Transaction<'_, DB>, list_id: &str) -> Result<List, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        create_list: &CreateList,
    ) -> Result<List, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_list: &UpdateList,
    ) -> Result<List, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, list_id: &str) -> Result<List, E>;
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
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        user_id: Option<&str>,
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
            WHERE user_id = COALESCE($4, user_id) AND (($2::text IS NULL AND $3::timestamptz IS NULL) OR (id, created_at) < ($2, $3))
            ORDER BY id DESC, created_at DESC
            LIMIT $1
            "#,
            limit,
            id,
            created_at,
            user_id
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        list_id: &str,
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
            ORDER BY created_at DESC
            "#n,
            list_id,
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

    async fn add_article(
        transaction: &mut Transaction<'_, Postgres>,
        list_id: &str,
        article_id: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            INSERT INTO listarticle (article_id, list_id)
            VALUES ($1, $2)
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
            DELETE FROM listarticle
            WHERE list_id = $1 AND article_id = $2
            "#n,
            list_id,
            article_id
        )
        .fetch_one(&mut **transaction)
        .await;
        Ok((list_id.to_string(), article_id.to_string()))
    }
}
