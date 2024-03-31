use super::enums::Role;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
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

pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub email_verified: Option<DateTime<Utc>>,
    pub image: Option<String>,
    pub password: Option<String>,
}

pub struct UpdateUser {
    pub id: i32,
    pub username: Option<String>,
    pub image: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct UserDTO {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub email_verified: Option<DateTime<Utc>>,
    pub image: Option<String>,
    pub role: Role,
    pub follower_count: i32,
    pub following_count: i32,
    pub approved_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl UserDTO {
    pub fn set(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            email_verified: user.email_verified,
            image: user.image,
            role: user.role,
            follower_count: user.follower_count,
            following_count: user.following_count,
            approved_at: user.approved_at,
            deleted_at: user.deleted_at,
        }
    }
}
