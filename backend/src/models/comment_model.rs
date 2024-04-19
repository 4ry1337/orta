use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub commenter_id: i32,
    pub article_id: Option<i32>,
    pub series_id: Option<i32>,
    pub list_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComment {
    pub user_id: i32,
    pub content: String,
    pub article_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateComment {
    pub id: i32,
    pub article_id: i32,
    pub user_id: i32,
    pub content: Option<String>,
}
