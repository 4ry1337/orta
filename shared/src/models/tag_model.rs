use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Postgres;

use super::enums::TagStatus;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: i32,
    pub label: String,
    pub article_count: i32,
    pub tag_status: TagStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl sqlx::Type<Postgres> for Tag {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("tags")
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for Tag {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode()?;
        let label = decoder.try_decode()?;
        let article_count = decoder.try_decode()?;
        let tag_status = decoder.try_decode()?;
        let created_at = decoder.try_decode()?;
        let updated_at = decoder.try_decode()?;
        Ok(Self {
            id,
            label,
            article_count,
            tag_status,
            created_at,
            updated_at,
        })
    }
}

pub struct CreateTag {
    pub label: String,
    pub tag_status: TagStatus,
}

pub struct UpdateTag {
    pub id: i32,
    pub label: Option<String>,
    pub tag_status: Option<TagStatus>,
}
