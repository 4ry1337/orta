use crate::{
    models::{
        article_model::{AddAuthor, Article, CreateArticle, DeleteAuthor, UpdateArticle},
        enums::Role,
        user_model::User,
    },
    utils::random_string::generate,
};
use async_trait::async_trait;
use slug::slugify;
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
    async fn find_all(transaction: &mut Transaction<'_, DB>) -> Result<Vec<Article>, E>;
    async fn find(transaction: &mut Transaction<'_, DB>, article_id: i32) -> Result<Article, E>;
    async fn find_by_authors(
        transaction: &mut Transaction<'_, DB>,
        users: &[i32],
    ) -> Result<Vec<Article>, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_article: &UpdateArticle,
    ) -> Result<Article, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, article_id: i32) -> Result<Article, E>;
    async fn get_authors(
        transaction: &mut Transaction<'_, DB>,
        article_id: i32,
    ) -> Result<Vec<User>, E>;
    async fn add_author(
        transaction: &mut Transaction<'_, DB>,
        add_author: &AddAuthor,
    ) -> Result<(i32, i32), E>;
    async fn delete_author(
        transaction: &mut Transaction<'_, DB>,
        delete_author: &DeleteAuthor,
    ) -> Result<(i32, i32), E>;
}

#[derive(Debug, Clone)]
pub struct ArticleRepositoryImpl;

#[async_trait]
impl ArticleRepository<Postgres, Error> for ArticleRepositoryImpl {
    async fn find_all(transaction: &mut Transaction<'_, Postgres>) -> Result<Vec<Article>, Error> {
        sqlx::query_as!(
            Article,
            r#"
            SELECT *
            FROM articles
            "#n
        )
        .fetch_all(&mut **transaction)
        .await
    }
    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: i32,
    ) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"
            SELECT *
            FROM articles
            WHERE id = $1
            "#n,
            article_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_by_authors(
        transaction: &mut Transaction<'_, Postgres>,
        users: &[i32],
    ) -> Result<Vec<Article>, Error> {
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
        .fetch_all(&mut **transaction)
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
            slugify(format!("{}-{}", create_article.title, generate(12))),
            create_article.title.trim()
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
                slug = COALESCE($3, articles.slug)
            WHERE articles.id = $1
            RETURNING *
            "#n,
            update_article.id,
            update_article.title,
            update_article
                .title
                .clone()
                .map(|v| slugify(format!("{}-{}", v, generate(12))))
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: i32,
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

    async fn get_authors(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: i32,
    ) -> Result<Vec<User>, Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                u.id,
                u.username,
                u.email,
                u.email_verified,
                u.image,
                u.role AS "role: Role",
                u.bio,
                u.urls,
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
        .fetch_all(&mut **transaction)
        .await
    }

    async fn add_author(
        transaction: &mut Transaction<'_, Postgres>,
        add_author: &AddAuthor,
    ) -> Result<(i32, i32), Error> {
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
        Ok((add_author.article_id, add_author.user_id))
    }

    async fn delete_author(
        transaction: &mut Transaction<'_, Postgres>,
        delete_author: &DeleteAuthor,
    ) -> Result<(i32, i32), Error> {
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
        Ok((delete_author.article_id, delete_author.user_id))
    }
}
