use serde::Serialize;

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
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
}

pub struct CreateAccount {
    pub user_id: i32,
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
}

pub struct UpdateAccount {
    pub id: i32,
    pub user_id: i32,
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
}
