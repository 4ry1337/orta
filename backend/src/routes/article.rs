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

pub async fn get_articles(State(state): State<Arc<AppState>>) -> Response {}

pub async fn get_articles_by_user(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let user_id = match params.user_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

pub async fn get_article(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

#[derive(Debug, Deserialize)]
pub struct PostArticleRequestBody {
    title: String,
    user_id: i32,
}

pub async fn post_article(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PostArticleRequestBody>,
) -> Response {
}

#[derive(Debug, Deserialize)]
pub struct PatchArticleRequestBody {
    pub title: Option<String>,
}

pub async fn patch_article(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchArticleRequestBody>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

pub async fn delete_article(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

pub async fn get_authors(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

pub async fn put_author(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let user_id = match params.user_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

pub async fn delete_author(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let user_id = match params.user_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}
