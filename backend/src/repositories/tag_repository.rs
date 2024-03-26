use axum::async_trait;
use sqlx::{Error, PgPool};

use crate::models::{
    enums::TagStatus,
    tag_model::{CreateTag, GetTags, Tag, UpdateTag},
};

#[async_trait]
pub trait TagRepository<E> {
    async fn find_all(&self, get_tags: GetTags) -> Result<Vec<Tag>, E>;
    async fn find(&self, tag_id: i32) -> Result<Tag, E>;
    async fn create(&self, create_tag: &CreateTag) -> Result<Tag, E>;
    async fn update(&self, update_tag: &UpdateTag) -> Result<Tag, E>;
    async fn delete(&self, tag_id: i32) -> Result<(), E>;
}

#[derive(Debug, Clone)]
pub struct PgTagRepository {
    db: PgPool,
}

impl PgTagRepository {
    pub fn new(db: PgPool) -> PgTagRepository {
        Self { db }
    }
}

#[async_trait]
impl TagRepository<Error> for PgTagRepository {
    async fn find_all(&self, get_tags: GetTags) -> Result<Vec<Tag>, Error> {
        match get_tags.tag_status {
            Some(tag_status) => {
                sqlx::query_as!(
                    Tag,
                    r#"
                    SELECT
                        id,
                        label,
                        article_count,
                        tag_status AS "tag_status: TagStatus",
                        created_at,
                        updated_at
                    FROM tags
                    WHERE tag_status = $1
                    "#n,
                    tag_status as TagStatus
                )
                .fetch_all(&self.db)
                .await
            }
            None => {
                sqlx::query_as!(
                    Tag,
                    r#"
                    SELECT
                        id,
                        label,
                        article_count,
                        tag_status AS "tag_status: TagStatus",
                        created_at,
                        updated_at
                    FROM tags
                    "#n
                )
                .fetch_all(&self.db)
                .await
            }
        }
    }

    async fn find(&self, tag_id: i32) -> Result<Tag, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT
                id,
                label,
                article_count,
                tag_status AS "tag_status: TagStatus",
                created_at,
                updated_at
            FROM tags
            WHERE id = $1
            "#n,
            tag_id
        )
        .fetch_one(&self.db)
        .await
    }

    async fn create(&self, create_tag: &CreateTag) -> Result<Tag, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            INSERT INTO tags (label)
            VALUES ($1)
            RETURNING
                id,
                label,
                article_count,
                tag_status AS "tag_status: TagStatus",
                created_at,
                updated_at
            "#n,
            create_tag.label
        )
        .fetch_one(&self.db)
        .await
    }

    async fn update(&self, update_tag: &UpdateTag) -> Result<Tag, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            UPDATE tags
            SET
                label = coalesce($2, tags.label),
                article_count = coalesce($3, tags.article_count),
                tag_status = coalesce($4, tags.tag_status)
            WHERE id = $1
            RETURNING
                id,
                label,
                article_count,
                tag_status AS "tag_status: TagStatus",
                created_at,
                updated_at
            "#n,
            update_tag.id,
            update_tag.label,
            update_tag.article_count,
            update_tag.tag_status as Option<TagStatus>
        )
        .fetch_one(&self.db)
        .await
    }

    async fn delete(&self, tag_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            DELETE FROM tags
            WHERE id = $1
            "#n,
            tag_id
        )
        .execute(&self.db)
        .await;
        Ok(())
    }
}
