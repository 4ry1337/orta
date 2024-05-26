use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Account {
    pub id: String,
    pub user_id: String,
    pub r#type: String,
    pub provider: String,
    pub provider_account_id: String,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<i64>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
    pub session_state: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct CreateAccount {
    pub user_id: String,
    pub r#type: String,
    pub provider: String,
    pub provider_account_id: String,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<i64>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
    pub session_state: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
}

pub struct UpdateAccount {
    pub id: String,
    pub user_id: String,
    pub r#type: Option<String>,
    pub provider: Option<String>,
    pub provider_account_id: Option<String>,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<i64>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
    pub session_state: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
}
