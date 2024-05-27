use async_trait::async_trait;
use chrono::{Days, Utc};
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::user_model::ValidationToken;

#[async_trait]
pub trait ValidationTokenRepository<DB, E>
where
    DB: Database,
{
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        user_id: &str,
    ) -> Result<ValidationToken, E>;
    async fn find(
        transaction: &mut Transaction<'_, DB>,
        user_id: &str,
    ) -> Result<ValidationToken, E>;
}

#[derive(Debug, Clone)]
pub struct ValidationTokenRepositoryImpl;

#[async_trait]
impl ValidationTokenRepository<Postgres, Error> for ValidationTokenRepositoryImpl {
    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &str,
    ) -> Result<ValidationToken, Error> {
        let _ = sqlx::query!(
            r#"
            DELETE FROM verification_token
            WHERE user_id = $1
            "#,
            user_id,
        )
        .execute(&mut **transaction)
        .await;

        sqlx::query_as!(
            ValidationToken,
            r#"
            INSERT INTO verification_token(user_id, expires_at)
            VALUES ($1, $2)
            RETURNING *
            "#,
            user_id,
            Utc::now().checked_add_days(Days::new(1))
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        token: &str,
    ) -> Result<ValidationToken, Error> {
        sqlx::query_as!(
            ValidationToken,
            r#"
            SELECT *
            FROM verification_token
            WHERE token = $1
            "#,
            token
        )
        .fetch_one(&mut **transaction)
        .await
    }
}
