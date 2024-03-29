use axum::async_trait;
use slug::slugify;
use sqlx::{postgres::PgQueryResult, Error, PgPool};

use crate::models::{
    enums::TagStatus,
    tag_model::{CreateTag, GetTags, Tag, UpdateTag},
};

#[async_trait]
pub trait TagRepository<E> {
    async fn find_all(&self, get_tags: &GetTags) -> Result<Vec<Tag>, E>;
    async fn find_by_article(&self, article_id: i32) -> Result<Vec<Tag>, E>;
    async fn find(&self, tag_id: i32) -> Result<Tag, E>;
    async fn create(&self, create_tag: &CreateTag) -> Result<Tag, E>;
    // async fn create_many(&self, create_tags: &[&CreateTag]) -> Result<Vec<Tag>, E>;
    async fn update(&self, update_tag: &UpdateTag) -> Result<Tag, E>;
    async fn delete(&self, tag_id: i32) -> Result<PgQueryResult, E>;
    async fn add_article_tags(&self, article_id: i32, tag_id: i32) -> Result<PgQueryResult, E>;
    async fn remove_article_tags(&self, article_id: i32, tag_id: i32) -> Result<PgQueryResult, E>;
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
    async fn find_all(&self, get_tags: &GetTags) -> Result<Vec<Tag>, Error> {
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
            WHERE tag_status = coalesce($1, tag_status)
            "#n,
            get_tags.tag_status as Option<TagStatus>
        )
        .fetch_all(&self.db)
        .await
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

    async fn find_by_article(&self, article_id: i32) -> Result<Vec<Tag>, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT t.id,
                t.label,
                t.article_count,
                t.tag_status AS "tag_status: TagStatus",
                t.created_at,
                t.updated_at
            FROM tags t
            JOIN articletags at ON t.id = at.tag_id
            WHERE at.article_id = $1 AND t.tag_status = 'APPROVED'
            ORDER BY at.created_at DESC
            "#n,
            article_id
        )
        .fetch_all(&self.db)
        .await
    }

    async fn create(&self, create_tag: &CreateTag) -> Result<Tag, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            INSERT INTO tags (label, tag_status)
            VALUES ($1, $2)
            RETURNING
                id,
                label,
                article_count,
                tag_status AS "tag_status: TagStatus",
                created_at,
                updated_at
            "#n,
            slugify(format!("{}", create_tag.label)),
            create_tag.tag_status as TagStatus
        )
        .fetch_one(&self.db)
        .await
    }

    // async fn create_many(&self, create_tags: &[&CreateTag]) -> Result<Vec<Tag>, Error> {
    //     let labels: Vec<String> = create_tags
    //         .iter()
    //         .map(|create_tag| create_tag.label.clone())
    //         .collect();
    //     let tag_statuses: Vec<TagStatus> = create_tags
    //         .iter()
    //         .map(|create_tag| create_tag.tag_status.clone())
    //         .collect();
    //     sqlx::query_as!(
    //         Tag,
    //         r#"
    //         INSERT INTO tags (label, tag_status)
    //         SELECT * FROM UNNEST ($1::text[],)
    //         RETURNING
    //             id,
    //             label,
    //             article_count,
    //             tag_status AS "tag_status: TagStatus",
    //             created_at,
    //             updated_at
    //         "#n,
    //         labels
    //     )
    //     .fetch_all(&self.db)
    //     .await
    // }

    async fn update(&self, update_tag: &UpdateTag) -> Result<Tag, Error> {
        let label = match &update_tag.label {
            Some(label) => Some(slugify(label)),
            None => None,
        };
        sqlx::query_as!(
            Tag,
            r#"
            UPDATE tags
            SET
                label = coalesce($2, tags.label),
                tag_status = coalesce($3, tags.tag_status)
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
            label,
            update_tag.tag_status as Option<TagStatus>
        )
        .fetch_one(&self.db)
        .await
    }

    async fn delete(&self, tag_id: i32) -> Result<PgQueryResult, Error> {
        sqlx::query!(
            r#"
            DELETE FROM tags
            WHERE id = $1
            "#n,
            tag_id
        )
        .execute(&self.db)
        .await
    }

    async fn add_article_tags(&self, article_id: i32, tag_id: i32) -> Result<PgQueryResult, Error> {
        sqlx::query!(
            r#"
            WITH article_count_increment AS (
                UPDATE tags
                SET article_count = article_count + 1
                WHERE id = $2
            )
            INSERT INTO articletags (article_id, tag_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#n,
            article_id,
            tag_id
        )
        .execute(&self.db)
        .await
    }

    async fn remove_article_tags(
        &self,
        article_id: i32,
        tag_id: i32,
    ) -> Result<PgQueryResult, Error> {
        sqlx::query!(
            r#"
            DELETE FROM articletags
            WHERE article_id = $1 AND tag_id = $2
            "#n,
            article_id,
            tag_id
        )
        .execute(&self.db)
        .await
    }
}
