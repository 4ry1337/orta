use async_trait::async_trait;
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::comment_model::{Comment, CreateComment, UpdateComment};

#[async_trait]
pub trait CommentRepository<DB, E>
where
    DB: Database,
{
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        article_id: i32,
    ) -> Result<Vec<Comment>, E>;
    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        comment_id: i32,
    ) -> Result<Comment, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        create_comment: &CreateComment,
    ) -> Result<Comment, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_comment: &UpdateComment,
    ) -> Result<Comment, E>;
    async fn delete(
        transaction: &mut Transaction<'_, DB>,
        comment_id: i32,
        article_id: i32,
    ) -> Result<Comment, E>;
}

#[derive(Debug, Clone)]
pub struct CommentRepositoryImpl;

#[async_trait]
impl CommentRepository<Postgres, Error> for CommentRepositoryImpl {
    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        article_id: i32,
    ) -> Result<Vec<Comment>, Error> {
        sqlx::query_as!(
            Comment,
            r#"
            SELECT *
            FROM comments
            WHERE article_id = $1
            "#n,
            article_id,
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        comment_id: i32,
    ) -> Result<Comment, Error> {
        sqlx::query_as!(
            Comment,
            r#"
            SELECT *
            FROM comments
            WHERE id = $1
            "#n,
            comment_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        create_comment: &CreateComment,
    ) -> Result<Comment, Error> {
        sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comments (
                commenter_id,
                content,
                article_id
            ) VALUES ($1, $2, $3)
            RETURNING *
            "#n,
            create_comment.user_id,
            create_comment.content,
            create_comment.article_id,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        update_comment: &UpdateComment,
    ) -> Result<Comment, Error> {
        sqlx::query_as!(
            Comment,
            r#"
            UPDATE comments
            SET content = COALESCE($4, comments.content)
            WHERE id = $1 AND commenter_id = $2 AND article_id = $3
            RETURNING *
            "#n,
            update_comment.id,
            update_comment.user_id,
            update_comment.article_id,
            update_comment.content
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        comment_id: i32,
        article_id: i32,
    ) -> Result<Comment, Error> {
        sqlx::query_as!(
            Comment,
            r#"
            DELETE FROM comments
            WHERE id = $1 AND article_id = $2
            RETURNING *
            "#n,
            comment_id,
            article_id
        )
        .fetch_one(&mut **transaction)
        .await
    }
}
