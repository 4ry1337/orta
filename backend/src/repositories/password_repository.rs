use axum::async_trait;
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::password_model::Password;

#[async_trait]
pub trait PasswordRepository<DB, E>
where
    DB: Database,
{
    async fn find(transaction: &mut Transaction<'_, DB>, user_id: i32) -> Result<Password, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        user_id: i32,
        salt: &str,
        password: &str,
    ) -> Result<Password, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        user_id: i32,
        password: &str,
        salt: &str,
    ) -> Result<Password, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, user_id: i32) -> Result<Password, E>;
}

#[derive(Debug, Clone)]
pub struct PasswordRepositoryImpl;

#[async_trait]
impl PasswordRepository<Postgres, Error> for PasswordRepositoryImpl {
    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
    ) -> Result<Password, Error> {
        sqlx::query_as!(
            Password,
            r#"
            SELECT *
            FROM passwords
            WHERE id = $1"#n,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
        password: &str,
        salt: &str,
    ) -> Result<Password, Error> {
        sqlx::query_as!(
            Password,
            r#"
            INSERT INTO passwords (id, password, salt)
            VALUES ($1, $2, $3)
            RETURNING *
            "#n,
            user_id,
            password,
            salt
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
        password: &str,
        salt: &str,
    ) -> Result<Password, Error> {
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
            password,
            salt
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
    ) -> Result<Password, Error> {
        sqlx::query_as!(
            Password,
            r#"
            DELETE FROM passwords
            WHERE id = $1
            RETURNING * 
            "#n,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }
}
