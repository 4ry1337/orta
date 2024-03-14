use std::error::Error;
use std::str::FromStr;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;

// #[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
// pub struct Account {
//     pub id: Option<i32>,
//     pub user_id: i32,
//     pub r#type: String,
//     pub provider: String,
//     pub provider_account_id: String,
//     pub refresh_token: String,
//     pub access_token: String,
//     pub expires_at: i64,
//     pub token_type: String,
//     pub scope: String,
//     pub id_token: String,
//     pub session_state: String,
// }

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
    Manager,
}

impl FromStr for Role {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<Role, Self::Err> {
        match input {
            "Admin" => Ok(Role::Admin),
            "User" => Ok(Role::User),
            "Manager" => Ok(Role::Manager),
            _ => Err(format!("Can not parse {} into Role Enum", input).into()),
        }
    }
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub email: String,
    pub email_verified: Option<DateTime<Utc>>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub role: Role,
    pub follower_count: i32,
    pub following_count: i32,
    pub approved_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl sqlx::Type<Postgres> for User {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("users")
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for User {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode()?;
        let username = decoder.try_decode()?;
        let email = decoder.try_decode()?;
        let email_verified = decoder.try_decode()?;
        let password = decoder.try_decode()?;
        let image = decoder.try_decode()?;
        let role = decoder.try_decode()?;
        let follower_count = decoder.try_decode()?;
        let following_count = decoder.try_decode()?;
        let approved_at = decoder.try_decode()?;
        let deleted_at = decoder.try_decode()?;
        Ok(Self {
            id,
            username,
            email,
            email_verified,
            password,
            image,
            role,
            follower_count,
            following_count,
            approved_at,
            deleted_at,
        })
    }
}

// #[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
// pub struct Device {
//     pub id: Option<String>,
//     pub last_logged: DateTime<Utc>,
//     pub device_data: String,
// }
