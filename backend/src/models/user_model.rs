use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Account {
    pub id: Option<i32>,
    #[allow(non_snake_case)]
    pub userId: i32,
    pub r#type: String,
    pub provider: String,
    #[allow(non_snake_case)]
    pub providerAccountId: String,
    pub refresh_token: String,
    pub access_token: String,
    pub expires_at: i64,
    pub token_type: String,
    pub scope: String,
    pub id_token: String,
    pub session_state: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Session {
    pub id: i32,
    #[allow(non_snake_case)]
    pub userId: i32,
    pub expires: DateTime<Utc>,
    #[allow(non_snake_case)]
    pub sessionToken: String,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
    Manager,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[allow(non_snake_case)]
    pub emailVerified: Option<DateTime<Utc>>,
    pub password: Option<String>,
    pub approved: Option<DateTime<Utc>>,
    pub image: Option<String>,
    pub bio: String,
    pub urls: Vec<String>,
    pub deleted: bool,
    pub following_count: i32,
    pub followers_count: i32,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Device {
    pub id: Option<String>,
    pub last_logged: DateTime<Utc>,
    pub device_data: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Publisher {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    #[allow(non_snake_case)]
    pub emailVerified: DateTime<Utc>,
    pub password: Option<String>,
    pub approved: DateTime<Utc>,
    pub image: Option<String>,
    pub bio: Option<String>,
    pub urls: Vec<String>,
    pub deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
