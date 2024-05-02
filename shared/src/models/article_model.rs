use crate::models::user_model::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub like_count: i32,
    pub comment_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct ArticleWithAuthors {
    pub id: i32,
    pub title: String,
    pub slug: String,
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

// #[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
// pub struct Device {
//     pub id: Option<String>,
//     pub last_logged: DateTime<Utc>,
//     pub device_data: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArticle {
    pub user_id: i32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArticle {
    pub id: i32,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddAuthor {
    pub user_id: i32,
    pub article_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAuthor {
    pub user_id: i32,
    pub article_id: i32,
}
