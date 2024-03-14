use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

use crate::models::article_model::Article;
use crate::models::user_model::*;

#[async_trait]
pub trait ArticleRepository<T, E> {
    fn set(db: T) -> Self;
    async fn create_article(&self, user_id: i32) -> Result<Article, E>;
    async fn get_articles(&self) -> Result<Vec<Article>, E>;
    async fn get_article_by_id(&self, article_id: i32) -> Result<Article, E>;
    async fn get_article_by_authors(&self, users: &[i32]) -> Result<Vec<Article>, E>;
    async fn update_article(&self, update_article: UpdateArticle) -> Result<Article, E>;
    async fn delete_article(&self, article_id: i32) -> Result<(), E>;
    async fn add_author(&self, add_author: AddAuthor) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct PgArticleRepository {
    db: PgPool,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct UpdateArticle {
    pub id: i32,
    pub title: Option<String>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct AddAuthor {
    pub user_id: i32,
    pub article_id: i32,
}

#[async_trait]
impl ArticleRepository<PgPool, Error> for PgArticleRepository {
    fn set(db: PgPool) -> PgArticleRepository {
        Self { db }
    }
    async fn get_articles(&self) -> Result<Vec<Article>, Error> {
        sqlx::query_as!(
            Article,
            r#"
            SELECT a.*,
                   array_agg(u.*) AS "authors: Vec<User>"
            FROM articles a
            JOIN authors au ON a.id = au.article_id
            JOIN users u ON u.id = au.author_id
            GROUP BY a.id
            "#n
        )
        .fetch_all(&self.db)
        .await
    }
    async fn get_article_by_id(&self, article_id: i32) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"
            SELECT a.*,
                   array_agg(u.*) AS "authors: Vec<User>"
            FROM articles a
            JOIN authors au ON a.id = au.article_id
            JOIN users u ON u.id = au.author_id
            WHERE a.id = $1
            GROUP BY a.id
            "#n,
            article_id
        )
        .fetch_one(&self.db)
        .await
    }

    async fn get_article_by_authors(&self, users: &[i32]) -> Result<Vec<Article>, Error> {
        sqlx::query_as!(
            Article,
            r#"
            SELECT a.*,
                   array_agg(u.*) AS "authors: Vec<User>"
            FROM articles a
            JOIN authors au ON a.id = au.article_id
            JOIN users u ON u.id = au.author_id
            GROUP BY a.id
            HAVING array_agg(u.id) @> $1;
            "#n,
            &users
        )
        .fetch_all(&self.db)
        .await
    }

    async fn create_article(&self, user_id: i32) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"
            WITH article AS
              (INSERT INTO articles DEFAULT
               VALUES RETURNING *),
                 author AS
              (INSERT INTO authors (author_id, article_id)
               VALUES ($1,
                         (SELECT id AS article_id
                          FROM article)) RETURNING *)
            SELECT a.*,
                   array_agg(u.*) AS "authors: Vec<User>"
            FROM articles a
            JOIN authors au ON a.id = au.article_id
            JOIN users u ON u.id = au.author_id
            WHERE a.id =
                (SELECT article_id
                 FROM article)
            GROUP BY a.id
            "#n,
            user_id
        )
        .fetch_one(&self.db)
        .await
    }

    async fn update_article(&self, update_article: UpdateArticle) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"
            WITH updated_article AS (
                UPDATE articles
                SET title = COALESCE($2, articles.title)
                WHERE articles.id = $1
                RETURNING *
            )
            SELECT
              a.*,
              array_agg(u.*) AS "authors: Vec<User>"
            FROM updated_article a
            JOIN authors au ON a.id = au.article_id
            JOIN users u ON u.id = au.author_id
            GROUP BY a.id, a.title, a.like_count, a.comment_count, a.created_at, a.published_at, a.updated_at
            "#n,
            update_article.id,
            update_article.title,
        )
        .fetch_one(&self.db)
        .await
    }

    async fn add_author(&self, add_author: AddAuthor) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            INSERT INTO authors (author_id, article_id)
            VALUES ($1, $2)
            "#n,
            add_author.user_id,
            add_author.article_id
        )
        .execute(&self.db)
        .await;
        Ok(())
    }

    async fn delete_article(&self, article_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            DELETE FROM articles
            WHERE id = $1
            "#n,
            article_id
        )
        .execute(&self.db)
        .await;
        Ok(())
    }
}
