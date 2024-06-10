use super::{enums::Visibility, user_model::FullUser};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct List {
    pub id: String,
    pub user_id: String,
    pub label: String,
    pub image: Option<String>,
    pub visibility: Visibility,
    pub article_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl sqlx::Type<Postgres> for List {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("lists")
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for List {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode()?;
        let user_id = decoder.try_decode()?;
        let label = decoder.try_decode()?;
        let image = decoder.try_decode()?;
        let visibility = decoder.try_decode()?;
        let article_count = decoder.try_decode()?;
        let created_at = decoder.try_decode()?;
        let updated_at = decoder.try_decode()?;
        Ok(Self {
            id,
            user_id,
            label,
            image,
            visibility,
            article_count,
            created_at,
            updated_at,
        })
    }
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct FullList {
    pub id: String,
    #[sqlx(flatten)]
    pub user: FullUser,
    pub label: String,
    pub image: Option<String>,
    pub visibility: Visibility,
    pub article_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateList {
    pub user_id: String,
    pub label: String,
    pub image: Option<String>,
    pub visibility: Visibility,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateList {
    pub id: String,
    pub label: Option<String>,
    pub image: Option<String>,
    pub visibility: Option<Visibility>,
}
