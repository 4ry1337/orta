use crate::models::user_model::*;
use axum::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[async_trait]
pub trait UserRepository<T, E> {
    fn set(db: T) -> Self;
    async fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>, E>;
    async fn get_user_by_email(&self, user_email: String) -> Result<Option<User>, E>;
    async fn create_user(&self, new_user: CreateUser) -> Result<User, E>;
    async fn update_user(&self, update_user: UpdateUser) -> Result<User, E>;
    async fn verify_user(&self, user_id: i32) -> Result<(), E>;
    async fn approve_user(&self, user_id: i32) -> Result<(), E>;
    async fn unapprove_user(&self, user_id: i32) -> Result<(), E>;
    async fn soft_delete_user(&self, user_id: i32) -> Result<(), E>;
    async fn delete_user(&self, user_id: i32) -> Result<(), E>;
}

#[derive(Debug, Clone)]
pub struct PgUserRepository {
    db: PgPool,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub email_verified: Option<DateTime<Utc>>,
    pub image: Option<String>,
    pub password: Option<String>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub id: i32,
    pub username: Option<String>,
    pub image: Option<String>,
}

#[async_trait]
impl UserRepository<PgPool, Error> for PgUserRepository {
    fn set(db: PgPool) -> PgUserRepository {
        Self { db }
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                username,
                email,
                email_verified,
                image,
                password,
                role AS "role: Role",
                follower_count,
                following_count,
                approved_at,
                deleted_at
            FROM users
            WHERE id = $1"#n,
            user_id
        )
        .fetch_optional(&self.db)
        .await
    }

    async fn get_user_by_email(&self, user_email: String) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                username,
                email,
                email_verified,
                image,
                password,
                role AS "role: Role",
                follower_count,
                following_count,
                approved_at,
                deleted_at
            FROM users
            WHERE email = $1
            "#n,
            user_email
        )
        .fetch_optional(&self.db)
        .await
    }

    async fn create_user(&self, new_user: CreateUser) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, email_verified, image, password)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                username,
                email,
                email_verified,
                image,
                password,
                role AS "role: Role",
                follower_count,
                following_count,
                approved_at,
                deleted_at
            "#n,
            new_user.username,
            new_user.email,
            new_user.email_verified,
            new_user.image,
            new_user.password,
        )
        .fetch_one(&self.db)
        .await
    }

    async fn update_user(&self, update_user: UpdateUser) -> Result<User, Error> {
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
                password,
                role AS "role: Role",
                follower_count,
                following_count,
                approved_at,
                deleted_at
            "#n,
            update_user.id,
            update_user.username,
            update_user.image
        )
        .fetch_one(&self.db)
        .await
    }

    async fn verify_user(&self, user_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
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
                password,
                role AS "role: Role",
                follower_count,
                following_count,
                approved_at,
                deleted_at
            "#n,
            user_id
        )
        .fetch_one(&self.db)
        .await;
        Ok(())
    }

    async fn approve_user(&self, user_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
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
                password,
                role AS "role: Role",
                follower_count,
                following_count,
                approved_at,
                deleted_at
            "#n,
            user_id
        )
        .fetch_one(&self.db)
        .await;
        Ok(())
    }

    async fn unapprove_user(&self, user_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
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
                password,
                role AS "role: Role",
                follower_count,
                following_count,
                approved_at,
                deleted_at
            "#n,
            user_id
        )
        .fetch_one(&self.db)
        .await;
        Ok(())
    }

    async fn soft_delete_user(&self, user_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            UPDATE users
            SET
                deleted_at = now()
            WHERE 
                id = $1
            "#n,
            user_id
        )
        .fetch_one(&self.db)
        .await;
        Ok(())
    }

    async fn delete_user(&self, user_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#n,
            user_id
        )
        .execute(&self.db)
        .await;
        Ok(())
    }
}
