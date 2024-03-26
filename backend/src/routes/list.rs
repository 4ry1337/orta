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
    models::{
        enums::Visibility,
        list_model::{CreateList, UpdateList},
    },
    repositories::list_repository::ListRepository,
    AppState,
};

pub async fn get_lists(State(state): State<Arc<AppState>>) -> Response {
    let response = state.repository.list.find_all().await;
    match response {
        Ok(lists) => (StatusCode::OK, Json(json!(lists))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct PostListRequestBody {
    pub user_id: i32,
    pub label: String,
    pub image: Option<String>,
    pub visibility: Visibility,
}

pub async fn post_list(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PostListRequestBody>,
) -> Response {
    let create_list = CreateList {
        user_id: payload.user_id,
        label: payload.label,
        image: payload.image,
        visibility: payload.visibility,
    };
    let response = state.repository.list.create(&create_list).await;
    match response {
        Ok(lists) => (StatusCode::OK, Json(json!(lists))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchListRequestBody {
    pub label: Option<String>,
    pub image: Option<String>,
    pub visibility: Option<Visibility>,
}

pub async fn patch_list(
    State(state): State<Arc<AppState>>,
    Path(list_id): Path<i32>,
    Json(payload): Json<PatchListRequestBody>,
) -> Response {
    let update_list = UpdateList {
        id: list_id,
        label: payload.label,
        image: payload.image,
        visibility: payload.visibility,
    };
    let response = state.repository.list.update(&update_list).await;
    match response {
        Ok(lists) => (StatusCode::OK, Json(json!(lists))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn delete_list(State(state): State<Arc<AppState>>, Path(list_id): Path<i32>) -> Response {
    let response = state.repository.list.delete(list_id).await;
    match response {
        Ok(()) => (StatusCode::OK, format!("Deleted article: {list_id}")).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn get_list_by_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Response {
    let response = state.repository.list.find_by_user(user_id).await;
    match response {
        Ok(lists) => (StatusCode::OK, Json(json!(lists))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn get_list_articles(
    State(state): State<Arc<AppState>>,
    Path(list_id): Path<i32>,
) -> Response {
    let response = state.repository.list.find_articles(list_id).await;
    match response {
        Ok(articles) => (StatusCode::OK, Json(json!(articles))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn post_list_article(
    State(state): State<Arc<AppState>>,
    Path(list_id): Path<i32>,
    Path(article_id): Path<i32>,
) -> Response {
    let response = state.repository.list.add_article(list_id, article_id).await;
    match response {
        Ok(()) => (
            StatusCode::OK,
            format!("Article {} added to List {}", article_id, list_id),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn delete_list_article(
    State(state): State<Arc<AppState>>,
    Path(list_id): Path<i32>,
    Path(article_id): Path<i32>,
) -> Response {
    let response = state
        .repository
        .list
        .remove_article(list_id, article_id)
        .await;
    match response {
        Ok(()) => (
            StatusCode::OK,
            format!("Article {} deleted to List {}", article_id, list_id),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}
