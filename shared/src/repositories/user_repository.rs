use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::{
    enums::Role,
    user_model::{CreateUser, FullUser, UpdateUser, User},
};

#[async_trait]
pub trait UserRepository<DB, E>
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
    ) -> Result<Vec<FullUser>, E>;
    async fn find_by_email(
        transaction: &mut Transaction<'_, DB>,
        user_email: &str,
    ) -> Result<User, E>;
    async fn find_by_username(
        transaction: &mut Transaction<'_, DB>,
        username: &str,
        by_user: Option<&str>,
    ) -> Result<FullUser, E>;
    async fn find_by_account(
        transaction: &mut Transaction<'_, DB>,
        account_provider: &str,
        account_id: &str,
    ) -> Result<User, E>;
    async fn find(transaction: &mut Transaction<'_, DB>, user_id: &str) -> Result<User, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        new_user: &CreateUser,
    ) -> Result<User, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_user: &UpdateUser,
    ) -> Result<User, E>;
    async fn verify(transaction: &mut Transaction<'_, DB>, user_id: &str) -> Result<User, E>;
    async fn approve(transaction: &mut Transaction<'_, DB>, user_id: &str) -> Result<User, E>;
    async fn unapprove(transaction: &mut Transaction<'_, DB>, user_id: &str) -> Result<User, E>;
    async fn soft_delete(transaction: &mut Transaction<'_, DB>, user_id: &str) -> Result<User, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, user_id: &str) -> Result<User, E>;
    async fn follow(
        transaction: &mut Transaction<'_, DB>,
        follower_id: &str,
        following_id: &str,
    ) -> Result<(String, String), E>;
    async fn unfollow(
        transaction: &mut Transaction<'_, DB>,
        follower_id: &str,
        following_id: &str,
    ) -> Result<(String, String), E>;
    async fn following(
        transaction: &mut Transaction<'_, DB>,
        user_id: &str,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullUser>, E>;
    async fn followers(
        transaction: &mut Transaction<'_, DB>,
        user_id: &str,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullUser>, E>;
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl;

#[async_trait]
impl UserRepository<Postgres, Error> for UserRepositoryImpl {
    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        _query: Option<&str>,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullUser>, Error> {
        sqlx::query_as!(
            FullUser,
            r#"
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
                    WHEN $4 IS NULL THEN FALSE
                    WHEN f.follower_id IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS followed
            FROM users u
            LEFT JOIN follow f ON u.id = f.following_id AND f.follower_id = $4
            WHERE (($2::timestamptz IS NULL AND $3::text IS NULL) OR (u.created_at, u.id) < ($2, $3))
            ORDER BY u.created_at DESC, u.id DESC 
            LIMIT $1
            "#,
            limit,
            created_at,
            id,
            by_user
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            FROM users
            WHERE id = $1"#,
            user_id,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_by_email(
        transaction: &mut Transaction<'_, Postgres>,
        user_email: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            FROM users
            WHERE email = $1
            "#,
            user_email
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_by_username(
        transaction: &mut Transaction<'_, Postgres>,
        username: &str,
        by_user: Option<&str>,
    ) -> Result<FullUser, Error> {
        sqlx::query_as!(
            FullUser,
            r#"
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
            WHERE u.username = $1
            "#,
            username,
            by_user
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_by_account(
        transaction: &mut Transaction<'_, Postgres>,
        account_provider: &str,
        account_id: &str,
    ) -> Result<User, Error> {
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
                u.created_at,
                u.approved_at,
                u.deleted_at
            FROM users u 
            JOIN accounts a ON u.id = a.user_id
            WHERE a.provider = $1 AND a.provider_account_id = $2
            "#,
            account_provider,
            account_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        create_user: &CreateUser,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, image)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            create_user.username,
            create_user.email,
            create_user.image,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        update_user: &UpdateUser,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                username = coalesce($2, users.username),
                image = coalesce($3, users.image)
            WHERE 
                id = $1
            RETURNING 
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            update_user.id,
            update_user.username,
            update_user.image
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn verify(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                email_verified = now()
            WHERE 
                id = $1
            RETURNING
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn approve(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                approved_at = now()
            WHERE 
                id = $1
            RETURNING
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn unapprove(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                approved_at = null
            WHERE 
                id = $1
            RETURNING
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn soft_delete(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                deleted_at = now()
            WHERE 
                id = $1
            RETURNING 
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING 
                id,
                username,
                email,
                email_verified,
                image,
                role AS "role: Role",
                bio,
                urls,
                follower_count,
                following_count,
                created_at,
                approved_at,
                deleted_at
            "#,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn follow(
        transaction: &mut Transaction<'_, Postgres>,
        follower_id: &str,
        following_id: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            WITH updated_follower AS (
                UPDATE users
                SET following_count = following_count + 1
                WHERE id = $1
                RETURNING id
            ),
            updated_following AS (
                UPDATE users
                SET follower_count = follower_count + 1
                WHERE id = $2
                RETURNING id
            )
            INSERT INTO follow (follower_id, following_id)
            SELECT updated_follower.id, updated_following.id
            FROM updated_follower, updated_following;
            "#,
            follower_id,
            following_id
        )
        .execute(&mut **transaction)
        .await;

        Ok((follower_id.to_string(), following_id.to_string()))
    }

    async fn unfollow(
        transaction: &mut Transaction<'_, Postgres>,
        follower_id: &str,
        following_id: &str,
    ) -> Result<(String, String), Error> {
        let _ = sqlx::query!(
            r#"
            WITH updated_follower AS (
                UPDATE users
                SET following_count = following_count - 1
                WHERE id = $1
                RETURNING id
            ),
            updated_following AS (
                UPDATE users
                SET follower_count = follower_count - 1
                WHERE id = $2
                RETURNING id
            )
            DELETE FROM follow
            WHERE follower_id = (SELECT id FROM updated_follower)
              AND following_id = (SELECT id FROM updated_following);
            "#,
            follower_id,
            following_id
        )
        .execute(&mut **transaction)
        .await;

        Ok((follower_id.to_string(), following_id.to_string()))
    }

    async fn following(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullUser>, Error> {
        sqlx::query_as!(
            FullUser,
            r#"
            WITH following AS (
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
                    u.deleted_at
                FROM follow f
                JOIN users u ON f.following_id = u.id
                WHERE f.follower_id = $1 AND (($3::timestamptz IS NULL AND $4::text IS NULL) OR (f.created_at, f.following_id) < ($3, $4))
                ORDER BY  f.created_at DESC, f.following_id DESC
                LIMIT $2
            )
            SELECT 
                fn.*,
                CASE
                    WHEN $5 IS NULL THEN FALSE
                    WHEN fo.follower_id IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS followed
            FROM following fn
            LEFT JOIN follow fo ON fn.id = fo.following_id AND fo.follower_id = $5
            "#,
            user_id,
            limit,
            created_at,
            id,
            by_user
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn followers(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
        limit: i64,
        id: Option<&str>,
        created_at: Option<DateTime<Utc>>,
        by_user: Option<&str>,
    ) -> Result<Vec<FullUser>, Error> {
        sqlx::query_as!(
            FullUser,
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
                    u.deleted_at
                FROM follow f
                LEFT JOIN users u ON f.following_id = u.id
                WHERE f.following_id = $1
                    AND (($3::timestamptz IS NULL AND $4::text IS NULL)
                        OR (f.created_at, f.follower_id) < ($3, $4))
                ORDER BY f.created_at DESC, f.follower_id DESC 
                LIMIT $2
            )
            SELECT 
                fs.*,
                CASE
                    WHEN $5 IS NULL THEN FALSE
                    WHEN fo.follower_id IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS followed
            FROM followers fs
            LEFT JOIN follow fo ON fs.id = fo.following_id AND fo.follower_id = $5
            "#,
            user_id,
            limit,
            created_at,
            id,
            by_user
        )
        .fetch_all(&mut **transaction)
        .await
    }
}
