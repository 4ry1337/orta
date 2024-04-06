use axum::async_trait;
use sqlx::{Error, PgPool};

use crate::{models::password_model::Password, utils::random_string::generate};

#[async_trait]
pub trait PasswordRepository<E> {
    async fn find(&self, user_id: i32) -> Result<Password, E>;
    async fn create(&self, user_id: i32, password: &str) -> Result<Password, E>;
    async fn update(&self, user_id: i32, password: &str) -> Result<Password, E>;
    async fn delete(&self, user_id: i32) -> Result<Password, E>;
}

#[derive(Debug, Clone)]
pub struct PgPasswordRepository {
    db: PgPool,
}

impl PgPasswordRepository {
    pub fn new(db: PgPool) -> PgPasswordRepository {
        Self { db }
    }
}

#[async_trait]
impl PasswordRepository<Error> for PgPasswordRepository {
    async fn find(&self, user_id: i32) -> Result<Password, Error> {
        sqlx::query_as!(
            Password,
            r#"
            SELECT *
            FROM passwords
            WHERE id = $1"#n,
            user_id
        )
        .fetch_one(&self.db)
        .await
    }

    async fn create(&self, user_id: i32, password: &str) -> Result<Password, Error> {
        let hashed_password = bcrypt::hash(password, 10).unwrap();
        let salt = generate(6);
        sqlx::query_as!(
            Password,
            r#"
            INSERT INTO passwords (id, password, salt)
            VALUES ($1, $2, $3)
            RETURNING *
            "#n,
            user_id,
            format!("{}{}", hashed_password, salt),
            salt
        )
        .fetch_one(&self.db)
        .await
    }

    async fn update(&self, user_id: i32, password: &str) -> Result<Password, Error> {
        let hashed_password = bcrypt::hash(password, 10).unwrap();
        let salt = generate(6);
        sqlx::query_as!(
            Password,
            r#"
            UPDATE passwords
            SET
                password = $2,
                salt = $3
            WHERE 
                id = $1
            RETURNING * 
            "#n,
            user_id,
            format!("{}{}", hashed_password, salt),
            salt
        )
        .fetch_one(&self.db)
        .await
    }

    async fn delete(&self, user_id: i32) -> Result<Password, Error> {
        sqlx::query_as!(
            Password,
            r#"
            DELETE FROM passwords
            WHERE id = $1
            RETURNING * 
            "#n,
            user_id
        )
        .fetch_one(&self.db)
        .await
    }
}
