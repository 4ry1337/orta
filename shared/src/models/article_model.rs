use crate::models::{list_model::List, series_model::Series, tag_model::Tag, user_model::FullUser};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
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
    pub description: Option<String>,
    pub like_count: i32,
    pub comment_count: i32,
    pub content: Option<String>,
    pub series: Option<Series>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub users: Option<Vec<FullUser>>,
    pub lists: Option<Vec<List>>,
    pub tags: Option<Vec<Tag>>,
    pub liked: Option<bool>,
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
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArticle {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
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
