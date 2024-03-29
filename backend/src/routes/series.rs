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

use crate::{
    models::series_model::{CreateSeries, UpdateSeries},
    repositories::{list_repository::ListRepository, series_repository::SeriesRepository},
    AppState,
};

pub async fn get_series(State(state): State<Arc<AppState>>) -> Response {
    let response = state.repository.series.find_all().await;
    match response {
        Ok(series) => (StatusCode::OK, Json(json!(series))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn get_series_by_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Response {
    let response = state.repository.series.find_by_user(user_id).await;
    match response {
        Ok(series) => (StatusCode::OK, Json(json!(series))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
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
    let create_series = CreateSeries {
        user_id: payload.user_id,
        label: payload.label,
        image: payload.image,
    };
    let response = state.repository.series.create(&create_series).await;
    match response {
        Ok(series) => (StatusCode::OK, Json(json!(series))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchSeriesRequestBody {
    pub label: Option<String>,
    pub image: Option<String>,
}

pub async fn patch_series(
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
    Json(payload): Json<PatchSeriesRequestBody>,
) -> Response {
    let update_series = UpdateSeries {
        id: series_id,
        label: payload.label,
        image: payload.image,
    };
    let response = state.repository.series.update(&update_series).await;
    match response {
        Ok(series) => (StatusCode::OK, Json(json!(series))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn delete_series(
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
) -> Response {
    let response = state.repository.series.delete(series_id).await;
    match response {
        Ok(()) => (StatusCode::OK, format!("Deleted series: {series_id}")).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn get_series_articles(
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
) -> Response {
    let response = state.repository.series.find_articles(series_id).await;
    match response {
        Ok(articles) => (StatusCode::OK, Json(json!(articles))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn put_series_article(
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
    Path(article_id): Path<i32>,
) -> Response {
    let response = state
        .repository
        .lists
        .add_article(series_id, article_id)
        .await;
    match response {
        Ok(()) => (
            StatusCode::OK,
            format!("Article {} added to Series {}", article_id, series_id),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn delete_series_article(
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
    Path(article_id): Path<i32>,
) -> Response {
    let response = state
        .repository
        .lists
        .remove_article(series_id, article_id)
        .await;
    match response {
        Ok(()) => (
            StatusCode::OK,
            format!("Article {} deleted to Series {}", article_id, series_id),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}
