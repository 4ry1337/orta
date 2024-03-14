use axum::async_trait;
use sqlx::{Error, PgPool};

use crate::models::tag_model::{Tag, TagStatus};

#[async_trait]
pub trait TagRepository<T, E> {
    fn set(db: T) -> Self;
    async fn get_tags(&self, get_tags: GetTags) -> Result<Vec<Tag>, E>;
    async fn create_tag(&self, new_tag: CreateTag) -> Result<Tag, E>;
    async fn update_tag(&self, update_tag: UpdateTag) -> Result<Tag, E>;
}

pub struct CreateTag {
    label: String,
}

pub struct UpdateTag {
    id: i32,
    label: Option<String>,
    article_count: Option<i32>,
    tag_status: Option<TagStatus>,
}

pub struct GetTags {
    tag_status: Option<TagStatus>,
}

#[derive(Debug, Clone)]
pub struct PgTagRepository {
    db: PgPool,
}

#[async_trait]
impl TagRepository<PgPool, Error> for PgTagRepository {
    fn set(db: PgPool) -> PgTagRepository {
        Self { db }
    }

    async fn get_tags(&self, get_tags: GetTags) -> Result<Vec<Tag>, Error> {
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

    async fn create_tag(&self, new_tag: CreateTag) -> Result<Tag, Error> {
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
            new_tag.label
        )
        .fetch_one(&self.db)
        .await
    }

    async fn update_tag(&self, update_tag: UpdateTag) -> Result<Tag, Error> {
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
}
