use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::{
    comment_model::{Comment, CreateComment, FullComment, UpdateComment},
    enums::CommentableType,
};

#[async_trait]
pub trait CommentRepository<DB, E>
where
    DB: Database,
{
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        query: Option<&str>,
        target_id: &str,
        r#type: CommentableType,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullComment>, E>;
    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        comment_id: &str,
        by_user: Option<&str>,
    ) -> Result<FullComment, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        create_comment: &CreateComment,
    ) -> Result<Comment, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_comment: &UpdateComment,
    ) -> Result<Comment, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, comment_id: &str) -> Result<Comment, E>;
}

#[derive(Debug, Clone)]
pub struct CommentRepositoryImpl;

#[async_trait]
impl CommentRepository<Postgres, Error> for CommentRepositoryImpl {
    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        _query: Option<&str>,
        target_id: &str,
        r#type: CommentableType,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullComment>, Error> {
        sqlx::query_as!(
            FullComment,
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
                        WHEN $6 IS NULL THEN FALSE
                        WHEN f.follower_id IS NOT NULL THEN TRUE
                        ELSE FALSE
                    END AS followed
                FROM users u
                LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $6
            )
            SELECT
                c.id,
                c.content,
                c.commenter_id,
                f.username,
                f.image,
                f.followed as "followed!: bool",
                c.target_id,
                c.type as "type: CommentableType",
                c.created_at,
                c.updated_at
            FROM comments c
            INNER JOIN followers f ON f.id = c.commenter_id
            WHERE (c.target_id = $1 AND c.type = $2) AND (($4::timestamptz IS NULL AND $5::text IS NULL) OR (c.created_at, c.id) < ($4, $5))
            ORDER BY c.created_at DESC, c.id DESC
            LIMIT $3
            "#n,
            target_id,
            r#type as CommentableType,
            limit,
            created_at,
            id,
            by_user,
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        comment_id: &str,
        by_user: Option<&str>,
    ) -> Result<FullComment, Error> {
        sqlx::query_as!(
            FullComment,
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
                        WHEN $2 IS NULL THEN FALSE
                        WHEN f.follower_id IS NOT NULL THEN TRUE
                        ELSE FALSE
                    END AS followed
                FROM users u
                LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $2
            )
            SELECT
                c.id,
                c.content,
                c.commenter_id,
                f.username,
                f.image,
                f.followed as "followed!: bool",
                c.target_id,
                c.type as "type: CommentableType",
                c.created_at,
                c.updated_at
            FROM comments c
            INNER JOIN followers f ON f.id = c.commenter_id
            WHERE c.id = $1
            "#n,
            comment_id,
            by_user
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
        comment_id: &str,
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
