use axum::async_trait;
use sqlx::{Error, PgPool};

use crate::models::{
    article_model::{AddAuthor, Article, CreateArticle, DeleteAuthor, UpdateArticle},
    enums::Role,
    user_model::User,
};

#[async_trait]
pub trait ArticleRepository<E> {
    async fn create(&self, create_article: &CreateArticle) -> Result<Article, E>;
    async fn find_all(&self) -> Result<Vec<Article>, E>;
    async fn find_by_id(&self, article_id: i32) -> Result<Article, E>;
    async fn find_by_authors(&self, users: &[i32]) -> Result<Vec<Article>, E>;
    async fn update(&self, update_article: &UpdateArticle) -> Result<Article, E>;
    async fn delete(&self, article_id: i32) -> Result<(), E>;
    async fn get_authors(&self, article_id: i32) -> Result<Vec<User>, E>;
    async fn add_author(&self, add_author: &AddAuthor) -> Result<(), E>;
    async fn delete_author(&self, delete_author: &DeleteAuthor) -> Result<(), E>;
}

#[derive(Debug, Clone)]
pub struct PgArticleRepository {
    db: PgPool,
}

impl PgArticleRepository {
    pub fn new(db: PgPool) -> PgArticleRepository {
        Self { db }
    }
}

#[async_trait]
impl ArticleRepository<Error> for PgArticleRepository {
    async fn find_all(&self) -> Result<Vec<Article>, Error> {
        sqlx::query_as!(
            Article,
            r#"
            SELECT *
            FROM articles
            "#n
        )
        .fetch_all(&self.db)
        .await
    }
    async fn find_by_id(&self, article_id: i32) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"
            SELECT *
            FROM articles
            WHERE id = $1
            "#n,
            article_id
        )
        .fetch_one(&self.db)
        .await
    }

    async fn find_by_authors(&self, users: &[i32]) -> Result<Vec<Article>, Error> {
        sqlx::query_as!(
            Article,
            r#"
            SELECT a.*
            FROM articles a
            JOIN authors au ON a.id = au.article_id
            GROUP BY a.id
            HAVING array_agg(au.author_id) @> $1;
            "#n,
            &users
        )
        .fetch_all(&self.db)
        .await
    }

    async fn create(&self, create_article: &CreateArticle) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"
            WITH article AS
              (INSERT INTO articles (slug, title)
               VALUES ($2, $3)
               RETURNING *),
                 author AS
              (INSERT INTO authors (author_id, article_id)
               VALUES ($1,
                         (SELECT id AS article_id
                          FROM article)) RETURNING *)
            SELECT *
            FROM article;
            "#n,
            create_article.user_id,
            create_article.title.trim().replace(" ", "-"),
            create_article.title.trim()
        )
        .fetch_one(&self.db)
        .await
    }

    async fn update(&self, update_article: &UpdateArticle) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"
            UPDATE articles
            SET title = COALESCE($2, articles.title)
            WHERE articles.id = $1
            RETURNING *
            "#n,
            update_article.id,
            update_article.title,
        )
        .fetch_one(&self.db)
        .await
    }

    async fn delete(&self, article_id: i32) -> Result<(), Error> {
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

    async fn get_authors(&self, article_id: i32) -> Result<Vec<User>, Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                u.id,
                u.username,
                u.email,
                u.email_verified,
                u.image,
                u.password,
                u.role AS "role: Role",
                u.follower_count,
                u.following_count,
                u.approved_at,
                u.deleted_at
            FROM authors a
            JOIN users u on a.author_id = u.id
            WHERE a.article_id = $1
            "#n,
            article_id
        )
        .fetch_all(&self.db)
        .await
    }

    async fn add_author(&self, add_author: &AddAuthor) -> Result<(), Error> {
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

    async fn delete_author(&self, delete_author: &DeleteAuthor) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            DELETE FROM authors
            WHERE author_id = $1 AND article_id = $2
            "#n,
            delete_author.user_id,
            delete_author.article_id
        )
        .execute(&self.db)
        .await;
        Ok(())
    }
}
