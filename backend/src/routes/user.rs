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
use shared::{models::prelude::*, repositories::prelude::*};
use tracing::error;

use crate::{application::AppState, utils::params::PathParams};

pub async fn get_users(State(state): State<Arc<AppState>>) -> Response {
    unimplemented!()
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let user_id = match params.user_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequestBody {
    pub username: String,
    pub email: String,
    pub email_verified: Option<DateTime<Utc>>,
    pub image: Option<String>,
}

pub async fn post_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequestBody>,
) -> Response {
}

#[derive(Debug, Deserialize)]
pub struct PatchUserRequestBody {
    pub username: Option<String>,
    pub image: Option<String>,
}

pub async fn patch_user(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchUserRequestBody>,
) -> Response {
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let user_id = match params.user_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}
