use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::user_model::{CreateUser, UpdateUser},
    repositories::user_repository::UserRepository,
    AppState,
};

pub async fn get_users(State(state): State<Arc<AppState>>) -> Response {
    let db_response = state.repository.user.find_all().await;
    match db_response {
        Ok(user) => (StatusCode::OK, Json(json!(user))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn get_user(State(state): State<Arc<AppState>>, Path(user_id): Path<i32>) -> Response {
    let db_response = state.repository.user.find_by_id(user_id).await;
    match db_response {
        Ok(user) => (StatusCode::OK, Json(json!(user))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequestBody {
    pub username: String,
    pub email: String,
    pub email_verified: Option<DateTime<Utc>>,
    pub image: Option<String>,
    pub password: Option<String>,
}

pub async fn post_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequestBody>,
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
    let create_user = CreateUser {
        username: payload.username,
        email: payload.email,
        image: payload.image,
        email_verified: payload.email_verified,
        password: hashed_password,
    };
    let db_response = state.repository.user.create(&create_user).await;
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
pub struct PatchUserRequestBody {
    pub username: Option<String>,
    pub image: Option<String>,
}

pub async fn patch_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
    Json(payload): Json<PatchUserRequestBody>,
) -> Response {
    let update_user = UpdateUser {
        id: user_id,
        username: payload.username,
        image: payload.image,
    };
    let db_response = state.repository.user.update(&update_user).await;
    match db_response {
        Ok(user) => (StatusCode::OK, Json(json!(user))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn delete_user(State(state): State<Arc<AppState>>, Path(user_id): Path<i32>) -> Response {
    let db_response = state.repository.user.delete(user_id).await;
    match db_response {
        Ok(()) => (StatusCode::OK, format!("Deleted user: {user_id}")).into_response(),
        Err(e) => (StatusCode::OK, Json(json!(e.to_string()))).into_response(),
    }
}
