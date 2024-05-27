use crate::models::{tag_model::Tag, user_model::User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub like_count: i32,
    pub comment_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct FullArticle {
    pub id: String,
    pub title: String,
    pub like_count: i32,
    pub comment_count: i32,
    pub content: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub users: Option<Vec<User>>,
    pub tags: Option<Vec<Tag>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct ArticleVersion {
    pub id: String,
    pub article_id: String,
    pub device_id: Option<String>,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Device {
    pub id: String,
    pub last_logged: DateTime<Utc>,
    pub device_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArticle {
    pub user_id: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArticle {
    pub id: String,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddAuthor {
    pub user_id: String,
    pub article_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAuthor {
    pub user_id: String,
    pub article_id: String,
}
