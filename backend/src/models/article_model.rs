use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Article {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub publisher_id: Option<i32>,
    pub user_ids: Vec<i32>,
    pub like_count: i32,
    pub comment_count: i32,
    pub tag_list: Vec<String>,
    pub reference: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct ArticleVersion {
    pub id: Option<String>,
    pub version_number: u32,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "block_type", rename_all = "lowercase")]
pub enum BlockType {
    Text,
    Image,
    Video,
    File,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct ArticleBlock {
    pub id: Option<String>,
    pub block_order: u8,
    pub block_type: BlockType,
    pub content: Option<String>,
}
