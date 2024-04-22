use async_trait::async_trait;
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::account_model::{Account, CreateAccount, UpdateAccount};

#[async_trait]
pub trait AccountRepository<DB, E>
where
    DB: Database,
{
    async fn find_all(transaction: &mut Transaction<'_, DB>) -> Result<Vec<Account>, E>;
    async fn find(transaction: &mut Transaction<'_, DB>, account_id: i32) -> Result<Account, E>;
    async fn find_by_user(
        transaction: &mut Transaction<'_, DB>,
        user_id: i32,
    ) -> Result<Account, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        new_account: &CreateAccount,
    ) -> Result<Account, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_accunt: &UpdateAccount,
    ) -> Result<Account, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, account_id: i32) -> Result<Account, E>;
}

#[derive(Debug, Clone)]
pub struct AccountRepositoryImpl;

#[async_trait]
impl AccountRepository<Postgres, Error> for AccountRepositoryImpl {
    async fn find_all(transaction: &mut Transaction<'_, Postgres>) -> Result<Vec<Account>, Error> {
        sqlx::query_as!(
            Account,
            r#"
            SELECT *
            FROM accounts
            "#
        )
        .fetch_all(&mut **transaction)
        .await
    }
    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        account_id: i32,
    ) -> Result<Account, Error> {
        sqlx::query_as!(
            Account,
            r#"
            SELECT *
            FROM accounts
            WHERE id = $1
            "#,
            account_id
        )
        .fetch_one(&mut **transaction)
        .await
    }
    async fn find_by_user(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
    ) -> Result<Account, Error> {
        sqlx::query_as!(
            Account,
            r#"
            SELECT *
            FROM accounts
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&mut **transaction)
        .await
    }
    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        new_account: &CreateAccount,
    ) -> Result<Account, Error> {
        sqlx::query_as!(
            Account,
            r#"
            INSERT INTO accounts
            (user_id, type, provider, provider_account_id, 
            refresh_token, access_token, expires_at,
            token_type, scope, id_token, session_state, password, salt)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
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
            new_account.password,
            new_account.salt
        )
        .fetch_one(&mut **transaction)
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
    async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        update_accunt: &UpdateAccount,
    ) -> Result<Account, Error> {
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
                session_state = coalesce($8,accounts.session_state),
                password = coalesce($9,accounts.password),
                salt = coalesce($10,accounts.salt)
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
            update_accunt.session_state,
            update_accunt.password,
            update_accunt.salt
        )
        .fetch_one(&mut **transaction)
        .await
    }
    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        account_id: i32,
    ) -> Result<Account, Error> {
        sqlx::query_as!(
            Account,
            r#"
            DELETE FROM accounts
            WHERE id = $1
            RETURNING *
            "#n,
            account_id
        )
        .fetch_one(&mut **transaction)
        .await
    }
}
