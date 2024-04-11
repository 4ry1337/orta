use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{patch, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::comment_model::{CreateComment, UpdateComment},
    repositories::{article_repository::ArticleRepository, comment_repository::CommentRepository},
    AppState,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/articles/:article_id/comments",
            post(post_comment).get(get_comments),
        )
        .route(
            "/articles/:article_id/comments/:comment_id",
            patch(patch_comment).delete(delete_comment),
        )
}

pub async fn get_comments(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Response {
    let response = state.repository.articles.find_by_id(article_id).await;

    if let Err(error) = response {
        if let sqlx::error::Error::RowNotFound = error {
            return (StatusCode::NOT_FOUND, "Article not found").into_response();
        }
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(error.to_string())),
        )
            .into_response();
    }

    let response = state
        .repository
        .comments
        .find_by_article_id(article_id)
        .await;

    match response {
        Ok(article) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct PostCommentRequestBody {
    user_id: i32,
    content: String,
}

pub async fn post_comment(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
    Json(payload): Json<PostCommentRequestBody>,
) -> Response {
    let response = state.repository.articles.find_by_id(article_id).await;

    if let Err(error) = response {
        if let sqlx::error::Error::RowNotFound = error {
            return (StatusCode::NOT_FOUND, "Article not found").into_response();
        }
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(error.to_string())),
        )
            .into_response();
    }

    let create_comment = CreateComment {
        article_id,
        user_id: payload.user_id,
        content: payload.content,
    };

    let db_response = state.repository.comments.create(&create_comment).await;

    match db_response {
        Ok(article) => (StatusCode::CREATED, Json(json!(article))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchCommentRequestBody {
    pub user_id: i32,
    pub content: String,
}

pub async fn patch_comment(
    State(state): State<Arc<AppState>>,
    Path((article_id, comment_id)): Path<(i32, i32)>,
    Json(payload): Json<PatchCommentRequestBody>,
) -> Response {
    let response = state.repository.articles.find_by_id(article_id).await;

    if let Err(error) = response {
        if let sqlx::error::Error::RowNotFound = error {
            return (StatusCode::NOT_FOUND, "Article not found").into_response();
        }
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(error.to_string())),
        )
            .into_response();
    }

    let update_comment = UpdateComment {
        id: comment_id,
        user_id: payload.user_id,
        content: payload.content,
    };

    let response = state.repository.comments.update(&update_comment).await;

    match response {
        Ok(article) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn delete_comment(
    State(state): State<Arc<AppState>>,
    Path((article_id, comment_id)): Path<(i32, i32)>,
) -> Response {
    let response = state.repository.articles.find_by_id(article_id).await;

    if let Err(error) = response {
        if let sqlx::error::Error::RowNotFound = error {
            return (StatusCode::NOT_FOUND, "Article not found").into_response();
        }
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(error.to_string())),
        )
            .into_response();
    }

    let response = state.repository.comments.delete(comment_id).await;

    match response {
        Ok(()) => (StatusCode::OK, format!("Deleted comment: {comment_id}")).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}
