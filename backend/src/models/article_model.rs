use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::user_model::User;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Article {
    pub id: i32,
    pub title: Option<String>,
    pub like_count: i32,
    pub comment_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub authors: Option<Vec<User>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct ArticleVersion {
    pub id: Option<String>,
    pub version_number: u32,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
