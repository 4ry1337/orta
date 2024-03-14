use std::error::Error;
use std::str::FromStr;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Postgres;

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "TagStatus")]
pub enum TagStatus {
    Approved,
    Banned,
    Waiting,
}

impl FromStr for TagStatus {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<TagStatus, Self::Err> {
        match input {
            "Approved" => Ok(TagStatus::Approved),
            "Banned" => Ok(TagStatus::Banned),
            "Waiting" => Ok(TagStatus::Waiting),
            _ => Err(format!("Can not parse {} into Tag Status Enum", input).into()),
        }
    }
}

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
