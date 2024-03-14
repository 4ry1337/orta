use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::{
    repository::user_repository::{CreateUser, UpdateUser, UserRepository},
    AppState,
};

pub async fn get_user(State(state): State<Arc<AppState>>, Path(user_id): Path<i32>) -> Response {
    let db_response = state.repository.user.get_user_by_id(user_id).await;
    match db_response {
        Ok(user) => (StatusCode::OK, Json(json!(user))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> Response {
    let hashed_password: Option<String> = match payload.password {
        Some(pass) => {
            let hash = bcrypt::hash(pass, 10);
            match hash {
                Ok(hash) => Some(hash),
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!(e.to_string())),
                    )
                        .into_response();
                }
            }
        }
        _ => Option::None,
    };
    let db_response = state
        .repository
        .user
        .create_user(CreateUser {
            password: hashed_password,
            ..payload
        })
        .await;
    match db_response {
        Ok(article) => (StatusCode::CREATED, Json(json!(article))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateUser>,
) -> Response {
    let db_response = state.repository.user.update_user(payload).await;
    match db_response {
        Ok(user) => (StatusCode::CREATED, Json(json!(user))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn delete_user(State(state): State<Arc<AppState>>, Path(user_id): Path<i32>) -> Response {
    let db_response = state.repository.user.delete_user(user_id).await;
    match db_response {
        Ok(()) => (StatusCode::OK, "Deleted user: {user_id}").into_response(),
        Err(e) => (StatusCode::OK, Json(json!(e.to_string()))).into_response(),
    }
}
