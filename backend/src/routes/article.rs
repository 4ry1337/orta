use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    Json,
};
use axum_core::response::IntoResponse;
use serde_json::json;

use crate::{
    repository::article_repository::{AddAuthor, ArticleRepository, UpdateArticle},
    AppState,
};

pub async fn get_article(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Response {
    let db_response = state.repository.article.get_article_by_id(article_id).await;
    match db_response {
        Ok(article) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn create_article(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Response {
    let db_response = state.repository.article.create_article(article_id).await;
    match db_response {
        Ok(article) => (StatusCode::CREATED, Json(json!(article))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn add_author(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AddAuthor>,
) -> Response {
    let db_response = state.repository.article.add_author(payload).await;
    match db_response {
        Ok(()) => (
            StatusCode::CREATED,
            "Author {payload.user_id} added to {payload.article_id}",
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn update_article(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateArticle>,
) -> Response {
    let db_response = state.repository.article.update_article(payload).await;
    match db_response {
        Ok(article) => (StatusCode::CREATED, Json(json!(article))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn delete_article(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Response {
    let db_response = state.repository.article.delete_article(article_id).await;
    match db_response {
        Ok(()) => (StatusCode::OK, "Deleted article: {article_id}").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}
