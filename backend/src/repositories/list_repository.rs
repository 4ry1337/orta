use axum::async_trait;
use sqlx::{Error, PgPool};

use crate::models::{
    article_model::Article,
    enums::Visibility,
    list_model::{CreateList, List, UpdateList},
};

#[async_trait]
pub trait ListRepository<T, E> {
    async fn find_all(&self) -> Result<Vec<List>, E>;
    async fn find_by_user(&self, user_id: i32) -> Result<Vec<List>, E>;
    async fn find(&self, list_id: i32) -> Result<List, E>;
    async fn create(&self, create_list: &CreateList) -> Result<List, E>;
    async fn update(&self, update_list: &UpdateList) -> Result<List, E>;
    async fn delete(&self, list_id: i32) -> Result<(), E>;
    async fn find_articles(&self, list_id: i32) -> Result<Vec<Article>, E>;
    async fn add_article(&self, list_id: i32, article_id: i32) -> Result<(), E>;
    async fn remove_article(&self, list_id: i32, article_id: i32) -> Result<(), E>;
}

#[derive(Debug, Clone)]
pub struct PgListRepository {
    db: PgPool,
}

impl PgListRepository {
    pub fn new(db: PgPool) -> PgListRepository {
        Self { db }
    }
}

#[async_trait]
impl ListRepository<PgPool, Error> for PgListRepository {
    async fn find_all(&self) -> Result<Vec<List>, Error> {
        sqlx::query_as!(
            List,
            r#"
            SELECT
                id,
                user_id,
                slug,
                label,
                image,
                visibility AS "visibility: Visibility",
                created_at,
                updated_at
            FROM lists
            ORDER BY created_at DESC
            "#n
        )
        .fetch_all(&self.db)
        .await
    }

    async fn find_by_user(&self, user_id: i32) -> Result<Vec<List>, Error> {
        sqlx::query_as!(
            List,
            r#"
            SELECT
                id,
                user_id,
                slug,
                label,
                image,
                visibility AS "visibility: Visibility",
                created_at,
                updated_at
            FROM lists
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#n,
            user_id,
        )
        .fetch_all(&self.db)
        .await
    }

    async fn find(&self, list_id: i32) -> Result<List, Error> {
        sqlx::query_as!(
            List,
            r#"
            SELECT
                id,
                user_id,
                slug,
                label,
                image,
                visibility AS "visibility: Visibility",
                created_at,
                updated_at
            FROM lists
            WHERE id = $1
            ORDER BY created_at DESC
            "#n,
            list_id,
        )
        .fetch_one(&self.db)
        .await
    }

    async fn create(&self, create_list: &CreateList) -> Result<List, Error> {
        let slug = create_list.label.clone().trim().replace(" ", "-");
        sqlx::query_as!(
            List,
            r#"
            INSERT INTO Lists (
                user_id,
                slug,
                label,
                image,
                visibility
                )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                user_id,
                slug,
                label,
                image,
                visibility AS "visibility: Visibility",
                created_at,
                updated_at
            "#n,
            create_list.user_id,
            slug,
            create_list.label,
            create_list.image,
            create_list.visibility as Visibility
        )
        .fetch_one(&self.db)
        .await
    }

    async fn update(&self, update_list: &UpdateList) -> Result<List, Error> {
        let slug = match update_list.label.clone() {
            Some(label) => label.clone().trim().replace(" ", "-"),
            None => "".to_string(),
        };
        sqlx::query_as!(
            List,
            r#"
            UPDATE lists
            SET slug = COALESCE($2, lists.slug),
                label = COALESCE($3, lists.label),
                image = COALESCE($4, lists.image),
                visibility = COALESCE($5, lists.visibility)
            WHERE id = $1
            RETURNING
                id,
                user_id,
                slug,
                label,
                image,
                visibility AS "visibility: Visibility",
                created_at,
                updated_at
            "#n,
            update_list.id,
            slug,
            update_list.label,
            update_list.image,
            update_list.visibility as Option<Visibility>
        )
        .fetch_one(&self.db)
        .await
    }

    async fn delete(&self, list_id: i32) -> Result<(), Error> {
        let _ = sqlx::query_as!(
            List,
            r#"
            DELETE FROM lists
            WHERE id = $1
            "#n,
            list_id,
        )
        .execute(&self.db)
        .await;
        Ok(())
    }

    async fn find_articles(&self, list_id: i32) -> Result<Vec<Article>, Error> {
        sqlx::query_as!(
            Article,
            r#"
            SELECT a.*
            FROM articles a
            JOIN listarticle la ON a.id = la.article_id
            WHERE la.list_id = $1
            ORDER BY la.created_at DESC
            "#n,
            list_id,
        )
        .fetch_all(&self.db)
        .await
    }

    async fn add_article(&self, list_id: i32, article_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            INSERT INTO listarticle (article_id, list_id)
            VALUES ($1, $2)
            "#n,
            article_id,
            list_id
        )
        .execute(&self.db)
        .await;
        Ok(())
    }

    async fn remove_article(&self, list_id: i32, article_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            DELETE FROM listarticle
            WHERE list_id = $1 AND article_id = $2
            "#n,
            list_id,
            article_id
        )
        .fetch_one(&self.db)
        .await;
        Ok(())
    }
}
