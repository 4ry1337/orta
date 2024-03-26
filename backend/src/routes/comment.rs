use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::comment_model::{CreateComment, UpdateComment},
    repositories::comment_repository::CommentRepository,
    AppState,
};

pub async fn get_comments(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Response {
    let db_response = state
        .repository
        .comment
        .find_by_article_id(article_id)
        .await;
    match db_response {
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
    let create_comment = CreateComment {
        article_id,
        user_id: payload.user_id,
        content: payload.content,
    };
    let db_response = state.repository.comment.create(&create_comment).await;
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
    Path(comment_id): Path<i32>,
    Json(payload): Json<PatchCommentRequestBody>,
) -> Response {
    let update_comment = UpdateComment {
        id: comment_id,
        user_id: payload.user_id,
        content: payload.content,
    };
    let db_response = state.repository.comment.update(&update_comment).await;
    match db_response {
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
    Path(comment_id): Path<i32>,
) -> Response {
    let db_response = state.repository.comment.delete(comment_id).await;
    match db_response {
        Ok(()) => (StatusCode::OK, format!("Deleted comment: {comment_id}")).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}
