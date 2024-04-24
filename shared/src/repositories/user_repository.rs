use async_trait::async_trait;
use sqlx::{Database, Error, Postgres, Transaction};

use crate::{
    models::{
        enums::Role,
        user_model::{CreateUser, UpdateUser, User},
    },
    utils::params::Filter,
};

#[async_trait]
pub trait UserRepository<DB, E>
where
    DB: Database,
{
    async fn total(transaction: &mut Transaction<'_, DB>) -> Result<Option<i64>, E>;
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        filters: &Filter,
    ) -> Result<Vec<User>, E>;
    async fn find_by_email(
        transaction: &mut Transaction<'_, DB>,
        user_email: &str,
    ) -> Result<User, E>;
    async fn find_by_username(
        transaction: &mut Transaction<'_, DB>,
        username: &str,
    ) -> Result<User, E>;
    async fn find_by_account(
        transaction: &mut Transaction<'_, DB>,
        account_provider: &str,
        account_id: &str,
    ) -> Result<User, E>;
    async fn find(transaction: &mut Transaction<'_, DB>, user_id: i32) -> Result<User, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        new_user: &CreateUser,
    ) -> Result<User, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_user: &UpdateUser,
    ) -> Result<User, E>;
    async fn verify(transaction: &mut Transaction<'_, DB>, user_id: i32) -> Result<User, E>;
    async fn approve(transaction: &mut Transaction<'_, DB>, user_id: i32) -> Result<User, E>;
    async fn unapprove(transaction: &mut Transaction<'_, DB>, user_id: i32) -> Result<User, E>;
    async fn soft_delete(transaction: &mut Transaction<'_, DB>, user_id: i32) -> Result<User, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, user_id: i32) -> Result<User, E>;
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl;

#[async_trait]
impl UserRepository<Postgres, Error> for UserRepositoryImpl {
    async fn total(transaction: &mut Transaction<'_, Postgres>) -> Result<Option<i64>, Error> {
        sqlx::query_scalar!("SELECT COUNT(*) FROM users")
            .fetch_one(&mut **transaction)
            .await
    }
    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        filters: &Filter,
    ) -> Result<Vec<User>, Error> {
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
                approved_at,
                deleted_at
            FROM users
            ORDER BY $1
            LIMIT $2
            OFFSET $3
            "#n,
            filters.order_by,
            filters.limit,
            filters.offset
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
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
                approved_at,
                deleted_at
            FROM users
            WHERE id = $1"#n,
            user_id
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
                approved_at,
                deleted_at
            FROM users
            WHERE email = $1
            "#n,
            user_email
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_by_username(
        transaction: &mut Transaction<'_, Postgres>,
        username: &str,
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
                approved_at,
                deleted_at
            FROM users
            WHERE username = $1
            "#n,
            username
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
                u.approved_at,
                u.deleted_at
            FROM users u 
            JOIN accounts a ON u.id = a.user_id
            WHERE a.provider = $1 AND a.provider_account_id = $2
            "#n,
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
            INSERT INTO users (username, email, email_verified, image)
            VALUES ($1, $2, $3, $4)
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
                approved_at,
                deleted_at
            "#n,
            create_user.username,
            create_user.email,
            create_user.email_verified,
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
                approved_at,
                deleted_at
            "#n,
            update_user.id,
            update_user.username,
            update_user.image
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn verify(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
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
                approved_at,
                deleted_at
            "#n,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn approve(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
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
                approved_at,
                deleted_at
            "#n,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn unapprove(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
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
                approved_at,
                deleted_at
            "#n,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn soft_delete(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
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
                approved_at,
                deleted_at
            "#n,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
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
                approved_at,
                deleted_at
            "#n,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }
}
