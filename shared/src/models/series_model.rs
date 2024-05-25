use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

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
