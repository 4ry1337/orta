use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres};

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Series {
    pub id: String,
    pub user_id: String,
    pub label: String,
    pub image: Option<String>,
    pub article_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl sqlx::Type<Postgres> for Series {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("series")
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for Series {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode()?;
        let user_id = decoder.try_decode()?;
        let label = decoder.try_decode()?;
        let image = decoder.try_decode()?;
        let article_count = decoder.try_decode()?;
        let created_at = decoder.try_decode()?;
        let updated_at = decoder.try_decode()?;
        Ok(Self {
            id,
            user_id,
            label,
            image,
            article_count,
            created_at,
            updated_at,
        })
    }
}

#[derive(Debug)]
pub struct CreateSeries {
    pub user_id: String,
    pub label: String,
    pub image: Option<String>,
}

#[derive(Debug)]
pub struct UpdateSeries {
    pub id: String,
    pub label: Option<String>,
    pub image: Option<String>,
}
