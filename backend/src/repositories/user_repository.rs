use axum::async_trait;
use sqlx::{Error, PgPool};

use crate::models::{
    enums::Role,
    user_model::{CreateUser, UpdateUser, User},
};

#[async_trait]
pub trait UserRepository<E> {
    async fn find_all(&self) -> Result<Vec<User>, E>;
    async fn find_by_id(&self, user_id: i32) -> Result<User, E>;
    async fn find_by_email(&self, user_email: String) -> Result<Option<User>, E>;
    async fn create(&self, new_user: &CreateUser) -> Result<User, E>;
    async fn update(&self, update_user: &UpdateUser) -> Result<User, E>;
    async fn verify(&self, user_id: i32) -> Result<(), E>;
    async fn approve(&self, user_id: i32) -> Result<(), E>;
    async fn unapprove(&self, user_id: i32) -> Result<(), E>;
    async fn soft_delete(&self, user_id: i32) -> Result<(), E>;
    async fn delete(&self, user_id: i32) -> Result<(), E>;
}

#[derive(Debug, Clone)]
pub struct PgUserRepository {
    db: PgPool,
}

impl PgUserRepository {
    pub fn new(db: PgPool) -> PgUserRepository {
        Self { db }
    }
}

#[async_trait]
impl UserRepository<Error> for PgUserRepository {
    async fn find_all(&self) -> Result<Vec<User>, Error> {
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
            "#n
        )
        .fetch_all(&self.db)
        .await
    }

    async fn find_by_id(&self, user_id: i32) -> Result<User, Error> {
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
        .fetch_one(&self.db)
        .await
    }

    async fn find_by_email(&self, user_email: String) -> Result<Option<User>, Error> {
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

    async fn create(&self, create_user: &CreateUser) -> Result<User, Error> {
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
            create_user.username,
            create_user.email,
            create_user.email_verified,
            create_user.image,
            create_user.password,
        )
        .fetch_one(&self.db)
        .await
    }

    async fn update(&self, update_user: &UpdateUser) -> Result<User, Error> {
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

    async fn verify(&self, user_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            UPDATE users
            SET
                email_verified = now()
            WHERE 
                id = $1
            "#n,
            user_id
        )
        .execute(&self.db)
        .await;
        Ok(())
    }

    async fn approve(&self, user_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            UPDATE users
            SET
                approved_at = now()
            WHERE 
                id = $1
            "#n,
            user_id
        )
        .execute(&self.db)
        .await;
        Ok(())
    }

    async fn unapprove(&self, user_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            UPDATE users
            SET
                approved_at = null
            WHERE 
                id = $1
            "#n,
            user_id
        )
        .execute(&self.db)
        .await;
        Ok(())
    }

    async fn soft_delete(&self, user_id: i32) -> Result<(), Error> {
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
        .execute(&self.db)
        .await;
        Ok(())
    }

    async fn delete(&self, user_id: i32) -> Result<(), Error> {
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
