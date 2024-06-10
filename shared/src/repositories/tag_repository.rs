use async_trait::async_trait;
use slug::slugify;
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::{
    enums::TagStatus,
    tag_model::{CreateTag, Tag, UpdateTag},
};

#[async_trait]
pub trait TagRepository<DB, E>
where
    DB: Database,
{
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        query: Option<&str>,
        limit: i64,
        tag_status: Option<TagStatus>,
        slug: Option<&str>,
    ) -> Result<Vec<Tag>, E>;
    async fn find(transaction: &mut Transaction<'_, DB>, tag_id: &str) -> Result<Tag, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        create_tag: &CreateTag,
    ) -> Result<Tag, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_tag: &UpdateTag,
    ) -> Result<Tag, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, tag_id: &str) -> Result<Tag, E>;
    async fn add_article_tags(
        transaction: &mut Transaction<'_, DB>,
        article_id: &str,
        tag_slug: &str,
    ) -> Result<(String, String), E>;
    async fn remove_article_tags(
        transaction: &mut Transaction<'_, DB>,
        article_id: &str,
    ) -> Result<String, E>;
    async fn add_user_tags(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
        tag_slug: &str,
    ) -> Result<(String, String), E>;
    async fn remove_user_tags(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<String, E>;
}

#[derive(Debug, Clone)]
pub struct TagRepositoryImpl;

#[async_trait]
impl TagRepository<Postgres, Error> for TagRepositoryImpl {
    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        query: Option<&str>,
        limit: i64,
        tag_status: Option<TagStatus>,
        slug: Option<&str>,
    ) -> Result<Vec<Tag>, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT
                t.label,
                t.slug,
                t.article_count,
                t.tag_status AS "tag_status: TagStatus",
                t.created_at,
                t.updated_at
            FROM tags t
            WHERE tag_status = coalesce($2, tag_status)
                AND (($3::text IS NULL) OR (slug > $3))
                AND (($4::text IS NULL) OR (label ILIKE $4))
            ORDER BY slug ASC
            LIMIT $1
            "#n,
            limit,
            tag_status as Option<TagStatus>,
            slug,
            query.map(|q| format!("%{}%", q))
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        tag_slug: &str,
    ) -> Result<Tag, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT
                label,
                slug,
                article_count,
                tag_status AS "tag_status: TagStatus",
                created_at,
                updated_at
            FROM tags
            WHERE slug = $1
            "#n,
            tag_slug
        )
        .fetch_one(&mut **transaction)
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
            ON CONFLICT DO NOTHING
            RETURNING
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
            WHERE slug = $1
            RETURNING
                label,
                slug,
                article_count,
                tag_status AS "tag_status: TagStatus",
                created_at,
                updated_at
            "#n,
            update_tag.slug,
            label,
            update_tag.tag_status as Option<TagStatus>,
            update_tag.label.clone().map(|v| slugify(v))
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        tag_id: &str,
    ) -> Result<Tag, Error> {
        sqlx::query_as!(
            Tag,
            r#"
            DELETE FROM tags
            WHERE slug = $1
            RETURNING
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
        article_id: &str,
        tag_slug: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            WITH article_count_increment AS (
                UPDATE tags
                SET article_count = article_count + 1
                WHERE slug = $2
            )
            INSERT INTO articletags (article_id, tag_slug)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#n,
            article_id,
            tag_slug
        )
        .execute(&mut **transaction)
        .await;

        Ok((article_id.to_string(), tag_slug.to_string()))
    }

    async fn remove_article_tags(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: &str,
    ) -> Result<String, Error> {
        let _ = sqlx::query!(
            r#"
            WITH article_count_decrement AS (
                UPDATE tags
                SET article_count = article_count - 1
                WHERE slug IN (
                    SELECT tag_slug
                    FROM articletags
                    WHERE article_id = $1
                )
            )
            DELETE FROM articletags
            WHERE article_id = $1
            "#n,
            article_id,
        )
        .execute(&mut **transaction)
        .await;

        Ok(article_id.to_string())
    }

    async fn add_user_tags(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
        tag_slug: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            INSERT INTO interests (user_id, tag_slug)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#n,
            user_id,
            tag_slug
        )
        .execute(&mut **transaction)
        .await;

        Ok((user_id.to_string(), tag_slug.to_string()))
    }

    async fn remove_user_tags(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<String, Error> {
        let _ = sqlx::query!(
            r#"
            WITH article_count_decrement AS (
                UPDATE tags
                SET article_count = article_count - 1
                WHERE slug IN (
                    SELECT tag_slug
                    FROM interests
                    WHERE user_id = $1
                )
            )
            DELETE FROM interests
            WHERE user_id = $1
            "#n,
            user_id,
        )
        .execute(&mut **transaction)
        .await;

        Ok(user_id.to_string())
    }
}
