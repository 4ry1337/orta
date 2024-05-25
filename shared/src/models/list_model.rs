use super::enums::Visibility;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

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
