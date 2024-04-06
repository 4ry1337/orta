use axum::async_trait;
use sqlx::{Error, PgPool};

use crate::models::account_model::{Account, CreateAccount, UpdateAccount};

#[async_trait]
pub trait AccountRepository<E> {
    async fn find_all(&self) -> Result<Vec<Account>, E>;
    async fn find(&self, account_id: i32) -> Result<Account, E>;
    async fn find_by_user(&self, user_id: i32) -> Result<Account, E>;
    async fn create(&self, new_account: &CreateAccount) -> Result<Account, E>;
    async fn update(&self, update_accunt: &UpdateAccount) -> Result<Account, E>;
    async fn delete(&self, account_id: i32) -> Result<Account, E>;
}

#[derive(Debug, Clone)]
pub struct PgAccountRepository {
    db: PgPool,
}

impl PgAccountRepository {
    pub fn new(db: PgPool) -> PgAccountRepository {
        Self { db }
    }
}

#[async_trait]
impl AccountRepository<Error> for PgAccountRepository {
    async fn find_all(&self) -> Result<Vec<Account>, Error> {
        sqlx::query_as!(
            Account,
            r#"
            SELECT *
            FROM accounts
            "#
        )
        .fetch_all(&self.db)
        .await
    }
    async fn find(&self, account_id: i32) -> Result<Account, Error> {
        sqlx::query_as!(
            Account,
            r#"
            SELECT *
            FROM accounts
            WHERE id = $1
            "#,
            account_id
        )
        .fetch_one(&self.db)
        .await
    }
    async fn find_by_user(&self, user_id: i32) -> Result<Account, Error> {
        sqlx::query_as!(
            Account,
            r#"
            SELECT *
            FROM accounts
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.db)
        .await
    }
    async fn create(&self, new_account: &CreateAccount) -> Result<Account, Error> {
        sqlx::query_as!(
            Account,
            r#"
            INSERT INTO accounts
            (user_id, type, provider, provider_account_id, 
            refresh_token, access_token, expires_at,
            token_type, scope, id_token, session_state)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#n,
            new_account.user_id,
            new_account.r#type,
            new_account.provider,
            new_account.provider_account_id,
            new_account.refresh_token,
            new_account.access_token,
            new_account.expires_at,
            new_account.token_type,
            new_account.scope,
            new_account.id_token,
            new_account.session_state,
        )
        .fetch_one(&self.db)
        .await
    }
    // CREATE TABLE verification_token
    // (
    //   identifier TEXT NOT NULL,
    //   expires TIMESTAMPTZ NOT NULL,
    //   token TEXT NOT NULL,
    //
    //   PRIMARY KEY (identifier, token)
    // );
    async fn update(&self, update_accunt: &UpdateAccount) -> Result<Account, Error> {
        sqlx::query_as!(
            Account,
            r#"
            UPDATE accounts
            SET 
                refresh_token = coalesce($2,accounts.refresh_token),
                access_token = coalesce($3,accounts.access_token),
                expires_at = coalesce($4,accounts.expires_at),
                token_type = coalesce($5,accounts.token_type),
                scope = coalesce($6,accounts.scope),
                id_token = coalesce($7,accounts.id_token),
                session_state = coalesce($8,accounts.session_state)
            WHERE id = $1
            RETURNING *
            "#n,
            update_accunt.id,
            update_accunt.refresh_token,
            update_accunt.access_token,
            update_accunt.expires_at,
            update_accunt.token_type,
            update_accunt.scope,
            update_accunt.id_token,
            update_accunt.session_state
        )
        .fetch_one(&self.db)
        .await
    }
    async fn delete(&self, account_id: i32) -> Result<Account, Error> {
        sqlx::query_as!(
            Account,
            r#"
            DELETE FROM accounts
            WHERE id = $1
            RETURNING *
            "#n,
            account_id
        )
        .fetch_one(&self.db)
        .await
    }
}
