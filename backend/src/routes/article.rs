use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_core::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;
use tracing::debug;

use crate::{
    repository::{article_repository::ArticleRepository, repository::QueryParamsImpl},
    AppState,
};

pub async fn get_article(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> impl IntoResponse {
    let db_response = state.repository.article.get_by_id(article_id).await;
    match db_response {
        Ok(article) => (StatusCode::OK, Json(json!(article))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        ),
    }
}

#[derive(Debug, Deserialize)]
pub struct GetArticleByUsers {
    pub users: Vec<i32>,
}

pub async fn get_articles_by_users(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GetArticleByUsers>,
) -> impl IntoResponse {
    let db_response = state.repository.article.get_by_users(&payload.users).await;
    match db_response {
        Ok(articles) => (StatusCode::OK, Json(json!(articles))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        ),
    }
}

pub async fn create_article(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    let db_response = state.repository.article.create(user_id).await;
    match db_response {
        Ok(article) => (StatusCode::CREATED, Json(json!(article))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        ),
    }
}
