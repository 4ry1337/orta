use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    Json,
};
use axum_core::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;
use shared::{models::prelude::*, repositories::prelude::*};
use tracing::error;

use crate::{application::AppState, utils::params::PathParams};

pub async fn get_series(State(state): State<Arc<AppState>>) -> Response {}

pub async fn get_series_by_id(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

#[derive(Debug, Deserialize)]
pub struct PostSeriesRequestBody {
    pub user_id: i32,
    pub label: String,
    pub image: Option<String>,
}

pub async fn post_series(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PostSeriesRequestBody>,
) -> Response {
}

#[derive(Debug, Deserialize)]
pub struct PatchSeriesRequestBody {
    pub label: Option<String>,
    pub image: Option<String>,
}

pub async fn patch_series(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchSeriesRequestBody>,
) -> Response {
    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

pub async fn delete_series(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

pub async fn put_series_article(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

pub async fn delete_series_article(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}
