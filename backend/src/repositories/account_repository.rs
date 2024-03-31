use axum::async_trait;
use sqlx::{Error, PgPool};

use crate::models::account_model::{Account, CreateAccount, UpdateAccount};

#[async_trait]
pub trait AccountRepository<E> {
    async fn find_all(&self) -> Result<Vec<Account>, E>;
    async fn find(&self, account_id: i32) -> Result<Account, E>;
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

// #[async_trait]
// impl AccountRepository<Error> for PgAccountRepository {
//     async fn find_all(&self) -> Result<Vec<Account>, Error>;
//     async fn find(&self, account_id: i32) -> Result<Account, Error>;
//     async fn create(&self, new_account: &CreateAccount) -> Result<Account, Error>;
//     async fn update(&self, update_accunt: &UpdateAccount) -> Result<Account, Error>;
//     async fn delete(&self, account_id: i32) -> Result<Account, Error>;
// }
