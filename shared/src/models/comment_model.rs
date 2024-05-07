use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::enums::CommentableType;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub commenter_id: i32,
    pub target_id: i32,
    pub r#type: CommentableType,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComment {
    pub user_id: i32,
    pub target_id: i32,
    pub content: String,
    pub r#type: CommentableType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateComment {
    pub id: i32,
    pub content: Option<String>,
}
