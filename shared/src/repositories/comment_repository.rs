use async_trait::async_trait;
use sqlx::{Database, Error, Postgres, Transaction};

use crate::{
    models::{
        comment_model::{Comment, CreateComment, UpdateComment},
        prelude::CommentableType,
    },
    utils::params::Filter,
};

#[async_trait]
pub trait CommentRepository<DB, E>
where
    DB: Database,
{
    async fn total(
        transaction: &mut Transaction<'_, DB>,
        target_id: i32,
        r#type: CommentableType,
    ) -> Result<Option<i64>, E>;
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        target_id: i32,
        r#type: CommentableType,
        filters: Filter,
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
    async fn delete(transaction: &mut Transaction<'_, DB>, comment_id: i32) -> Result<Comment, E>;
}

#[derive(Debug, Clone)]
pub struct CommentRepositoryImpl;

#[async_trait]
impl CommentRepository<Postgres, Error> for CommentRepositoryImpl {
    async fn total(
        transaction: &mut Transaction<'_, Postgres>,
        target_id: i32,
        r#type: CommentableType,
    ) -> Result<Option<i64>, Error> {
        sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM comments
            WHERE target_id = $1 AND type = $2
            "#n,
            target_id,
            r#type as CommentableType
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        target_id: i32,
        r#type: CommentableType,
        filters: Filter,
    ) -> Result<Vec<Comment>, Error> {
        sqlx::query_as!(
            Comment,
            r#"
            SELECT
                id,
                content,
                commenter_id,
                target_id,
                type as "type: CommentableType",
                created_at,
                updated_at
            FROM comments
            WHERE target_id = $1 AND type = $2
            ORDER BY created_at ASC
            LIMIT $3
            OFFSET $4
            "#n,
            target_id,
            r#type as CommentableType,
            filters.limit,
            filters.offset
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
            SELECT
                id,
                content,
                commenter_id,
                target_id,
                type as "type: CommentableType",
                created_at,
                updated_at
            FROM comments
            WHERE id = $1
            "#n,
            comment_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    // SELECT * FROM (
    //     -- Query to get comments by a specific commenter_id
    //     SELECT *
    //     FROM Comments
    //     WHERE commenter_id = $1
    //
    //     UNION ALL
    //
    //     -- Query to get remaining comments (excluding those by the specific commenter_id)
    //     SELECT *
    //     FROM Comments
    //     WHERE commenter_id != $1
    // ) AS combined_comments
    // ORDER BY created_at;

    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        create_comment: &CreateComment,
    ) -> Result<Comment, Error> {
        sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comments (
                commenter_id,
                target_id,
                type,
                content
            ) VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                content,
                commenter_id,
                target_id,
                type as "type: CommentableType",
                created_at,
                updated_at
            "#n,
            create_comment.user_id,
            create_comment.target_id,
            create_comment.r#type as CommentableType,
            create_comment.content
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
            SET content = COALESCE($2, comments.content)
            WHERE id = $1
            RETURNING
                id,
                content,
                commenter_id,
                target_id,
                type as "type: CommentableType",
                created_at,
                updated_at
            "#n,
            update_comment.id,
            update_comment.content
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        comment_id: i32,
    ) -> Result<Comment, Error> {
        sqlx::query_as!(
            Comment,
            r#"
            DELETE FROM comments
            WHERE id = $1
            RETURNING
                id,
                content,
                commenter_id,
                target_id,
                type as "type: CommentableType",
                created_at,
                updated_at
            "#n,
            comment_id,
        )
        .fetch_one(&mut **transaction)
        .await
    }
}
