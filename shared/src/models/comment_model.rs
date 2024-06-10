use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::enums::CommentableType;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: String,
    pub content: String,
    pub commenter_id: String,
    pub target_id: String,
    pub r#type: CommentableType,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct FullComment {
    pub id: String,
    pub content: String,
    pub commenter_id: String,
    pub username: String,
    pub image: Option<String>,
    pub followed: bool,
    pub target_id: String,
    pub r#type: CommentableType,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComment {
    pub user_id: String,
    pub target_id: String,
    pub content: String,
    pub r#type: CommentableType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateComment {
    pub id: String,
    pub content: Option<String>,
}
