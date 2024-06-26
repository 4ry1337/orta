use crate::models::{
    article_model::{
        AddAuthor, Article, ArticleVersion, CreateArticle, DeleteAuthor, FullArticle, UpdateArticle,
    },
    list_model::List,
    series_model::Series,
    tag_model::Tag,
    user_model::FullUser,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Database, Error, Postgres, Transaction};

#[async_trait]
pub trait ArticleRepository<DB, E>
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
        published: Option<bool>,
    ) -> Result<Vec<FullArticle>, E>;
    async fn find(
        transaction: &mut Transaction<'_, DB>,
        article_id: &str,
        by_user: Option<&str>,
    ) -> Result<FullArticle, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        create_article: &CreateArticle,
    ) -> Result<Article, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_article: &UpdateArticle,
    ) -> Result<Article, E>;
    async fn like(
        transaction: &mut Transaction<'_, DB>,
        article_id: &str,
        user_id: &str,
    ) -> Result<(String, String), E>;
    async fn unlike(
        transaction: &mut Transaction<'_, DB>,
        article_id: &str,
        user_id: &str,
    ) -> Result<(String, String), E>;
    async fn publish(transaction: &mut Transaction<'_, DB>, article_id: &str)
        -> Result<Article, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, article_id: &str) -> Result<Article, E>;
    async fn add_author(
        transaction: &mut Transaction<'_, DB>,
        add_author: &AddAuthor,
    ) -> Result<(String, String), E>;
    async fn delete_author(
        transaction: &mut Transaction<'_, DB>,
        delete_author: &DeleteAuthor,
    ) -> Result<(String, String), E>;
    async fn history(
        transaction: &mut Transaction<'_, DB>,
        article_id: &str,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Vec<ArticleVersion>, E>;
    async fn version(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: &str,
    ) -> Result<ArticleVersion, E>;
    async fn edit(
        transaction: &mut Transaction<'_, DB>,
        article_id: &str,
        content: &str,
        device_id: Option<&str>,
    ) -> Result<ArticleVersion, E>;
}

#[derive(Debug, Clone)]
pub struct ArticleRepositoryImpl;

#[async_trait]
impl ArticleRepository<Postgres, Error> for ArticleRepositoryImpl {
    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        query: Option<&str>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
        published: Option<bool>,
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
                        WHEN $5 IS NULL THEN FALSE
                        WHEN f.follower_id IS NOT NULL THEN TRUE
                        ELSE FALSE
                    END AS followed
                FROM users u
                LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $5
            )
            SELECT
                a.*,
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT f.*) FILTER (WHERE f.id IS NOT NULL), NULL) as "users: Vec<FullUser>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT t.*) FILTER (WHERE t.slug IS NOT NULL), NULL) as "tags: Vec<Tag>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT s.*) FILTER (WHERE s.id IS NOT NULL), null) as "series: Vec<Series>",
                CASE
                    WHEN $5::text is NULL then ARRAY[]::lists[]
                    ELSE ARRAY_REMOVE(ARRAY_AGG(DISTINCT l.*) FILTER (WHERE l.id IS NOT NULL), NULL)
                END AS "lists: Vec<List>",
                CASE
                    WHEN $5::text IS NULL THEN FALSE
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
            WHERE ($6::bool IS NULL OR (
                    CASE 
                        WHEN $6 IS TRUE THEN a.published_at IS NOT NULL
                        ELSE a.published_at IS NULL
                    END)
                ) AND (($2::TEXT IS NULL AND $3::TIMESTAMPTZ IS NULL) OR (a.created_at, a.id) < ($3, $2))
                AND (($4::TEXT IS NULL)
                    OR (to_tsvector(a.title || ' ' || COALESCE(a.description, '') || ' ' || COALESCE(a.content, ''))
                        @@ websearch_to_tsquery($4)))
            GROUP BY a.id, s.id, li.user_id, sa.order
            ORDER BY a.created_at DESC, a.id DESC
            LIMIT $1
            "#n,
            limit,
            id,
            created_at,
            query,
            by_user,
            published,
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: &str,
        by_user: Option<&str>,
    ) -> Result<FullArticle, Error> {
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
                        WHEN f.follower_id IS NOT NULL THEN TRUE
                        ELSE FALSE
                    END AS followed
                FROM users u
                LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $2
            )
            SELECT
                a.*,
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT f.*) FILTER (WHERE f.id IS NOT NULL), NULL) as "users: Vec<FullUser>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT t.*) FILTER (WHERE t.slug IS NOT NULL), NULL) as "tags: Vec<Tag>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT s.*) FILTER (WHERE s.id IS NOT NULL), null) as "series: Vec<Series>",
                CASE
                    WHEN $2::text is NULL then ARRAY[]::lists[]
                    ELSE ARRAY_REMOVE(ARRAY_AGG(DISTINCT l.*) FILTER (WHERE l.id IS NOT NULL), NULL)
                END AS "lists: Vec<List>",
                CASE
                    WHEN li.user_id IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS liked,
                sa.order AS "order: Option<f32>"
            FROM articles a
            LEFT JOIN likes li ON a.id = li.article_id AND li.user_id = $2
            LEFT JOIN authors au ON a.id = au.article_id
            LEFT JOIN followers f ON au.author_id = f.id
            LEFT JOIN articletags at ON a.id = at.article_id
            LEFT JOIN tags t ON at.tag_slug = t.slug
            LEFT JOIN seriesarticle sa ON a.id = sa.article_id
            LEFT JOIN series s ON sa.series_id = s.id
            LEFT JOIN listarticle la ON a.id = la.article_id
            LEFT JOIN lists l ON la.list_id = l.id AND l.user_id = $2
            WHERE a.id = $1
            GROUP BY a.id, li.user_id, sa.order
            "#n,
            article_id,
            by_user,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        create_article: &CreateArticle,
    ) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"
            WITH article AS (
                INSERT INTO articles (title, description)
                VALUES ($2, $3)
                RETURNING *
            ), author AS (
                INSERT INTO authors (author_id, article_id, is_owner)
                VALUES ($1, (
                    SELECT id AS article_id
                    FROM article
                ), true)
                RETURNING *
            )
            SELECT *
            FROM article a
            "#n,
            create_article.user_id,
            create_article.title,
            create_article.description
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        update_article: &UpdateArticle,
    ) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"
            UPDATE articles
            SET
                title = COALESCE($2, articles.title),
                description = COALESCE($3, articles.description)
            WHERE articles.id = $1
            RETURNING *
            "#n,
            update_article.id,
            update_article.title,
            update_article.description
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: &str,
    ) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"
            DELETE FROM articles
            WHERE id = $1
            RETURNING *
            "#n,
            article_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn add_author(
        transaction: &mut Transaction<'_, Postgres>,
        add_author: &AddAuthor,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            INSERT INTO authors (author_id, article_id)
            VALUES ($1, $2)
            "#n,
            add_author.user_id,
            add_author.article_id
        )
        .execute(&mut **transaction)
        .await;
        Ok((
            add_author.article_id.to_string(),
            add_author.user_id.to_string(),
        ))
    }

    async fn delete_author(
        transaction: &mut Transaction<'_, Postgres>,
        delete_author: &DeleteAuthor,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            DELETE FROM authors
            WHERE author_id = $1
                AND article_id = $2
            "#n,
            delete_author.user_id,
            delete_author.article_id
        )
        .execute(&mut **transaction)
        .await;
        Ok((
            delete_author.article_id.to_string(),
            delete_author.user_id.to_string(),
        ))
    }

    async fn history(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: &str,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Vec<ArticleVersion>, Error> {
        sqlx::query_as!(
            ArticleVersion,
            r#"
            SELECT *
            FROM articleversions
            WHERE article_id = $4
                AND (($2::text IS NULL AND $3::timestamptz IS NULL)
                    OR (id, created_at) < ($2, $3))
            ORDER BY id DESC, created_at DESC
            LIMIT $1
            "#n,
            limit,
            id,
            created_at,
            article_id,
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn version(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: &str,
    ) -> Result<ArticleVersion, Error> {
        sqlx::query_as!(
            ArticleVersion,
            r#"
            SELECT *
            FROM articleversions
            WHERE article_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#n,
            article_id,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn edit(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: &str,
        content: &str,
        device_id: Option<&str>,
    ) -> Result<ArticleVersion, Error> {
        sqlx::query_as!(
            ArticleVersion,
            r#"
            INSERT INTO articleversions (article_id, content, device_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#n,
            article_id,
            content,
            device_id,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn like(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: &str,
        user_id: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query_as!(
            Article,
            r#"
            WITH article AS (
                UPDATE articles
                SET like_count = like_count + 1
                WHERE id = $2
                RETURNING id
            )
            INSERT INTO likes (user_id, article_id)
            VALUES ($1, (SELECT id FROM article))
            "#n,
            user_id,
            article_id,
        )
        .fetch_one(&mut **transaction)
        .await;
        Ok((user_id.to_string(), article_id.to_string()))
    }

    async fn unlike(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: &str,
        user_id: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            WITH article AS (
                UPDATE articles
                SET like_count = like_count - 1
                WHERE id = $2
                RETURNING id
            )
            DELETE FROM likes
            WHERE user_id = $1 AND article_id = (SELECT id FROM article)
            "#,
            user_id,
            article_id,
        )
        .execute(&mut **transaction)
        .await;

        Ok((user_id.to_string(), article_id.to_string()))
    }

    async fn publish(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: &str,
    ) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"
            WITH lav AS (
                SELECT content
                FROM articleversions
                WHERE article_id = $1
                ORDER BY created_at DESC
                LIMIT 1
            )
            UPDATE articles
            SET published_at = now(),
                content = lav.content
            FROM lav
            WHERE articles.id = $1
            RETURNING articles.*
            "#n,
            article_id,
        )
        .fetch_one(&mut **transaction)
        .await
    }
}
