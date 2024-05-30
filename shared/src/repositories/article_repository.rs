use crate::models::{
    article_model::{
        AddAuthor, Article, ArticleVersion, CreateArticle, DeleteAuthor, FullArticle, UpdateArticle,
    },
    tag_model::Tag,
    user_model::User,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Database, Error, Postgres, Transaction};

#[async_trait]
pub trait ArticleRepository<DB, E>
where
    DB: Database,
{
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        create_article: &CreateArticle,
    ) -> Result<Article, E>;
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        usernames: Vec<String>,
        list_id: Option<&str>,
        series_id: Option<&str>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Vec<FullArticle>, E>;
    async fn find(
        transaction: &mut Transaction<'_, DB>,
        article_id: &str,
    ) -> Result<FullArticle, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_article: &UpdateArticle,
    ) -> Result<Article, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, article_id: &str) -> Result<Article, E>;
    async fn add_author(
        transaction: &mut Transaction<'_, DB>,
        add_author: &AddAuthor,
    ) -> Result<(String, String), E>;
    async fn delete_author(
        transaction: &mut Transaction<'_, DB>,
        delete_author: &DeleteAuthor,
    ) -> Result<(String, String), E>;
    async fn versions(
        transaction: &mut Transaction<'_, DB>,
        article_id: &str,
    ) -> Result<Option<i64>, E>;
    async fn history(
        transaction: &mut Transaction<'_, DB>,
        article_id: &str,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Vec<ArticleVersion>, E>;
    async fn save(
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
        usernames: Vec<String>,
        _list_id: Option<&str>,
        _series_id: Option<&str>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Vec<FullArticle>, Error> {
        sqlx::query_as!(
            FullArticle,
            r#"
            WITH latest_articleversions AS (
                SELECT av.article_id, av.content
                FROM articleversions av
                INNER JOIN (
                    SELECT article_id, MAX(created_at) AS max_created_at
                    FROM articleversions
                    GROUP BY article_id
                ) latest_av ON av.article_id = latest_av.article_id AND av.created_at = latest_av.max_created_at
            )
            SELECT
                a.*,
                lav.content as "content: Option<String>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT u.*) FILTER (WHERE u.id IS NOT NULL), NULL) as "users: Vec<User>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT t.*) FILTER (WHERE t.slug IS NOT NULL), NULL) as "tags: Vec<Tag>"
            FROM articles a
            LEFT JOIN authors au ON a.id = au.article_id
            LEFT JOIN users u ON au.author_id = u.id
            LEFT JOIN articletags at ON a.id = at.article_id
            LEFT JOIN tags t ON at.tag_slug = t.slug
            LEFT JOIN latest_articleversions lav ON a.id = lav.article_id
            WHERE (($2::text IS NULL AND $3::timestamptz IS NULL) OR (a.id, a.created_at) < ($2, $3))
            GROUP BY a.id, lav.content
            HAVING array_agg(u.username) @> $4
            ORDER BY a.id DESC, a.created_at DESC
            LIMIT $1
            "#n,
            limit,
            id,
            created_at,
            &usernames,
        )
        .fetch_all(&mut **transaction)
        .await
    }

    // LEFT JOIN listarticle     la ON a.id         = la.article_id
    // LEFT JOIN seriesarticle   sa ON a.id         = sa.article_id

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: &str,
    ) -> Result<FullArticle, Error> {
        sqlx::query_as!(
            FullArticle,
            r#"
            WITH latest_articleversions AS (
                SELECT av.article_id, av.content
                FROM articleversions av
                INNER JOIN (
                    SELECT article_id, MAX(created_at) AS max_created_at
                    FROM articleversions
                    GROUP BY article_id
                ) latest_av ON av.article_id = latest_av.article_id AND av.created_at = latest_av.max_created_at
            )
            SELECT
                a.*,
                lav.content as "content: Option<String>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT u.*) FILTER (WHERE u.id IS NOT NULL), NULL) as "users: Vec<User>",
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT t.*) FILTER (WHERE t.slug IS NOT NULL), NULL) as "tags: Vec<Tag>"
            FROM articles a
            LEFT JOIN authors au ON a.id = au.article_id
            LEFT JOIN users u ON au.author_id = u.id
            LEFT JOIN articletags at ON a.id = at.article_id
            LEFT JOIN tags t ON at.tag_slug = t.slug
            LEFT JOIN latest_articleversions lav ON a.id = lav.article_id
            WHERE a.id = $1
            GROUP BY a.id, lav.content
            "#n,
            article_id
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
            WITH article AS
              (INSERT INTO articles (title, description)
               VALUES ($2, $3)
               RETURNING *),
                 author AS
              (INSERT INTO authors (author_id, article_id)
               VALUES ($1,
                         (SELECT id AS article_id
                          FROM article)) RETURNING *)
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
            SET title = COALESCE($2, articles.title),
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
            WHERE author_id = $1 AND article_id = $2
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

    async fn versions(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: &str,
    ) -> Result<Option<i64>, Error> {
        sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM articleversions
            GROUP BY article_id = $1
            "#,
            article_id
        )
        .fetch_one(&mut **transaction)
        .await
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
            WHERE article_id = $4 AND (($2::text IS NULL AND $3::timestamptz IS NULL) OR (id, created_at) < ($2, $3))
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

    async fn save(
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
}
