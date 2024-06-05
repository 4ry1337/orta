use super::enums::Role;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Postgres;

#[derive(Clone, sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub email_verified: Option<DateTime<Utc>>,
    pub image: Option<String>,
    pub role: Role,
    pub bio: String,
    pub urls: Vec<String>,
    pub follower_count: i32,
    pub following_count: i32,
    pub created_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl sqlx::Type<Postgres> for User {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("user")
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
        let image = decoder.try_decode()?;
        let role = decoder.try_decode::<Role>()?;
        let bio = decoder.try_decode()?;
        let urls = decoder.try_decode()?;
        let follower_count = decoder.try_decode()?;
        let following_count = decoder.try_decode()?;
        let created_at = decoder.try_decode()?;
        let approved_at = decoder.try_decode()?;
        let deleted_at = decoder.try_decode()?;
        Ok(Self {
            id,
            username,
            email,
            email_verified,
            image,
            role,
            bio,
            urls,
            follower_count,
            following_count,
            created_at,
            approved_at,
            deleted_at,
        })
    }
}

#[derive(Clone, sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct FullUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub email_verified: Option<DateTime<Utc>>,
    pub image: Option<String>,
    // pub role: Role,
    pub bio: String,
    pub urls: Vec<String>,
    pub follower_count: i32,
    pub following_count: i32,
    pub created_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub followed: Option<bool>,
}

impl sqlx::Type<Postgres> for FullUser {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("fulluser")
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for FullUser {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode()?;
        let username = decoder.try_decode()?;
        let email = decoder.try_decode()?;
        let email_verified = decoder.try_decode()?;
        let image = decoder.try_decode()?;
        // let role = decoder.try_decode::<Role>()?;
        let bio = decoder.try_decode()?;
        let urls = decoder.try_decode()?;
        let follower_count = decoder.try_decode()?;
        let following_count = decoder.try_decode()?;
        let created_at = decoder.try_decode()?;
        let approved_at = decoder.try_decode()?;
        let deleted_at = decoder.try_decode()?;
        let followed = decoder.try_decode()?;
        Ok(Self {
            id,
            username,
            email,
            email_verified,
            image,
            // role,
            bio,
            urls,
            follower_count,
            following_count,
            created_at,
            approved_at,
            deleted_at,
            followed,
        })
    }
}

pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub image: Option<String>,
}

pub struct UpdateUser {
    pub id: String,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub urls: Vec<String>,
}

#[derive(Clone, sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct ValidationToken {
    pub token: String,
    pub user_id: String,
    pub expires_at: DateTime<Utc>,
}
