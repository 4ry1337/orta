use async_trait::async_trait;
use slug::slugify;
use sqlx::{Database, Error, Postgres, Transaction};

use crate::{
    models::{
        enums::TagStatus,
        tag_model::{CreateTag, Tag, UpdateTag},
    },
    utils::params::Filter,
};

#[async_trait]
pub trait TagRepository<DB, E>
where
    DB: Database,
{
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        tag_status: Option<TagStatus>,
        filters: &Filter,
    ) -> Result<Vec<Tag>, E>;
    async fn find_by_user(
        transaction: &mut Transaction<'_, DB>,
        user_id: i32,
    ) -> Result<Vec<Tag>, E>;
    async fn find_by_article(
        transaction: &mut Transaction<'_, DB>,
        article_id: i32,
    ) -> Result<Vec<Tag>, E>;
    async fn find(transaction: &mut Transaction<'_, DB>, tag_id: i32) -> Result<Tag, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        create_tag: &CreateTag,
    ) -> Result<Tag, E>;
    // async fn create_many(transaction: &mut Transaction<'_, DB>, create_tags: &[&CreateTag]) -> Result<Vec<Tag>, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_tag: &UpdateTag,
    ) -> Result<Tag, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, tag_id: i32) -> Result<Tag, E>;
    async fn add_article_tags(
        transaction: &mut Transaction<'_, DB>,
        article_id: i32,
        tag_id: i32,
    ) -> Result<(), E>;
    async fn remove_article_tags(
        transaction: &mut Transaction<'_, DB>,
        article_id: i32,
        tag_id: i32,
    ) -> Result<(), E>;
}

#[derive(Debug, Clone)]
pub struct TagRepositoryImpl;

#[async_trait]
impl TagRepository<Postgres, Error> for TagRepositoryImpl {
    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        tag_status: Option<TagStatus>,
        filters: &Filter,
    ) -> Result<Vec<Tag>, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT
                id,
                label,
                slug,
                article_count,
                tag_status AS "tag_status: TagStatus",
                created_at,
                updated_at
            FROM tags
            WHERE tag_status = coalesce($1, tag_status)
            ORDER BY $2
            LIMIT $3
            OFFSET $4
            "#n,
            tag_status as Option<TagStatus>,
            filters.order_by,
            filters.limit,
            filters.offset
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(transaction: &mut Transaction<'_, Postgres>, tag_id: i32) -> Result<Tag, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT
                id,
                label,
                slug,
                article_count,
                tag_status AS "tag_status: TagStatus",
                created_at,
                updated_at
            FROM tags
            WHERE id = $1
            "#n,
            tag_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_by_user(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
    ) -> Result<Vec<Tag>, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT
                t.id,
                t.label,
                t.slug,
                t.article_count,
                t.tag_status AS "tag_status: TagStatus",
                t.created_at,
                t.updated_at
            FROM tags t
            JOIN interests i ON t.id = i.tag_id
            WHERE i.user_id = $1 AND t.tag_status = 'APPROVED'
            ORDER BY i.created_at DESC
            "#n,
            user_id
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find_by_article(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: i32,
    ) -> Result<Vec<Tag>, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT
                t.id,
                t.label,
                t.slug,
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
        .fetch_all(&mut **transaction)
        .await
    }

    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        create_tag: &CreateTag,
    ) -> Result<Tag, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            INSERT INTO tags (label, slug, tag_status)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                label,
                slug,
                article_count,
                tag_status AS "tag_status: TagStatus",
                created_at,
                updated_at
            "#n,
            create_tag.label,
            slugify(format!("{}", create_tag.label)),
            create_tag.tag_status as TagStatus
        )
        .fetch_one(&mut **transaction)
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

    async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        update_tag: &UpdateTag,
    ) -> Result<Tag, Error> {
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
                tag_status = coalesce($3, tags.tag_status),
                slug = coalesce($4, tags.slug)
            WHERE id = $1
            RETURNING
                id,
                label,
                slug,
                article_count,
                tag_status AS "tag_status: TagStatus",
                created_at,
                updated_at
            "#n,
            update_tag.id,
            label,
            update_tag.tag_status as Option<TagStatus>,
            update_tag.label.clone().map(|v| slugify(v))
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        tag_id: i32,
    ) -> Result<Tag, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            DELETE FROM tags
            WHERE id = $1
            RETURNING
                id,
                label,
                slug,
                article_count,
                tag_status AS "tag_status: TagStatus",
                created_at,
                updated_at
            "#n,
            tag_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn add_article_tags(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: i32,
        tag_id: i32,
    ) -> Result<(), Error> {
        let _ = sqlx::query!(
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
        .execute(&mut **transaction)
        .await;
        Ok(())
    }

    async fn remove_article_tags(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: i32,
        tag_id: i32,
    ) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            DELETE FROM articletags
            WHERE article_id = $1 AND tag_id = $2
            "#n,
            article_id,
            tag_id
        )
        .execute(&mut **transaction)
        .await;
        Ok(())
    }
}