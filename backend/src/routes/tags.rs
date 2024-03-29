use std::{str::FromStr, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::{
        enums::TagStatus,
        tag_model::{CreateTag, GetTags, UpdateTag},
    },
    repositories::tag_repository::TagRepository,
    AppState,
};

#[derive(Deserialize)]
pub struct GetTagsQuery {
    tag_status: Option<String>,
}

pub async fn get_tags(
    State(state): State<Arc<AppState>>,
    get_tags_query: Query<GetTagsQuery>,
) -> Response {
    let mut get_tags = GetTags { tag_status: None };
    if let Some(tag_status) = &get_tags_query.tag_status {
        if let Ok(tag_status) = TagStatus::from_str(&tag_status) {
            get_tags.tag_status = Some(tag_status);
        }
    }
    let db_response = state.repository.tags.find_all(&get_tags).await;
    match db_response {
        Ok(tags) => (StatusCode::OK, Json(json!(tags))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn get_tag(State(state): State<Arc<AppState>>, Path(tag_id): Path<i32>) -> Response {
    let db_response = state.repository.tags.find(tag_id).await;
    match db_response {
        Ok(tag) => (StatusCode::OK, Json(json!(tag))).into_response(),
        Err(error) => {
            if let sqlx::error::Error::RowNotFound = error {
                return (StatusCode::NOT_FOUND, "Tag not found").into_response();
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostTagRequestBody {
    pub label: String,
    pub tag_status: Option<String>,
}

pub async fn post_tag(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PostTagRequestBody>,
) -> Response {
    let mut create_tag = CreateTag {
        label: payload.label,
        tag_status: TagStatus::Waiting,
    };
    if let Some(tag_status_str) = &payload.tag_status {
        if let Ok(tag_status) = TagStatus::from_str(&tag_status_str) {
            println!("{:?}", tag_status);
            create_tag.tag_status = tag_status;
        }
    };
    let response = state.repository.tags.create(&create_tag).await;
    match response {
        Ok(label) => (StatusCode::CREATED, Json(json!(label))).into_response(),
        Err(error) => {
            if let Some(database_error) = error.as_database_error() {
                if let Some(constraint) = database_error.constraint() {
                    if constraint == "tags_label_key" {
                        return (
                            StatusCode::BAD_REQUEST,
                            format!(r#"Tag: "{}" already exists"#, create_tag.label),
                        )
                            .into_response();
                    }
                }
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchTagRequestBody {
    pub label: Option<String>,
    pub tag_status: Option<String>,
}

pub async fn patch_tag(
    State(state): State<Arc<AppState>>,
    Path(tag_id): Path<i32>,
    Json(payload): Json<PatchTagRequestBody>,
) -> Response {
    let mut update_tag = UpdateTag {
        id: tag_id,
        label: payload.label,
        tag_status: None,
    };
    if let Some(tag_status) = &payload.tag_status {
        if let Ok(tag_status) = TagStatus::from_str(&tag_status) {
            update_tag.tag_status = Some(tag_status);
        }
    };
    let response = state.repository.tags.update(&update_tag).await;
    match response {
        Ok(article) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(error) => {
            if let Some(label) = update_tag.label {
                if let Some(database_error) = error.as_database_error() {
                    if let Some(constraint) = database_error.constraint() {
                        if constraint == "tags_label_key" {
                            return (
                                StatusCode::BAD_REQUEST,
                                format!(r#"Tag: "{}" already exists"#, label),
                            )
                                .into_response();
                        }
                    }
                }
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response()
        }
    }
}

pub async fn delete_tag(State(state): State<Arc<AppState>>, Path(tag_id): Path<i32>) -> Response {
    let response = state.repository.tags.delete(tag_id).await;
    match response {
        Ok(response) => {
            if response.rows_affected() == 0 {
                return (
                    StatusCode::BAD_REQUEST,
                    format!("Tag id: {tag_id} does not exists"),
                )
                    .into_response();
            }
            (StatusCode::OK, format!("Deleted tag id: {tag_id}")).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}
