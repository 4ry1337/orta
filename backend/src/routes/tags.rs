use std::{str::FromStr, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use shared::{models::prelude::*, repositories::prelude::*};

use tracing::error;

use crate::{application::AppState, utils::params::PathParams};

#[derive(Deserialize)]
pub struct GetTagsQuery {
    tag_status: Option<String>,
}

pub async fn get_tags(
    State(state): State<Arc<AppState>>,
    get_tags_query: Query<GetTagsQuery>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let tags = match TagRepositoryImpl::find_all(
        &mut transaction,
        &GetTags {
            tag_status: get_tags_query
                .tag_status
                .as_ref()
                .and_then(|s| TagStatus::from_str(s).ok()),
        },
    )
    .await
    {
        Ok(tags) => tags,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(tags))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn get_tag(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let tag_id = match params.tag_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let tag = match TagRepositoryImpl::find(&mut transaction, tag_id).await {
        Ok(tag) => tag,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "Tag not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(tag))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
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
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let tag = match TagRepositoryImpl::create(
        &mut transaction,
        &CreateTag {
            label: payload.label,
            tag_status: payload
                .tag_status
                .as_ref()
                .and_then(|s| TagStatus::from_str(s).ok())
                .unwrap_or(TagStatus::Waiting),
        },
    )
    .await
    {
        Ok(tag) => tag,
        Err(err) => {
            error!("{:#?}", err);
            if let Some(database_error) = err.as_database_error() {
                if let Some(constraint) = database_error.constraint() {
                    if constraint == "tags_label_key" {
                        return (StatusCode::BAD_REQUEST, "Tag already exists").into_response();
                    }
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match transaction.commit().await {
        Ok(_) => (StatusCode::CREATED, Json(json!(tag))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
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
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchTagRequestBody>,
) -> Response {
    let tag_id = match params.tag_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let tag = match TagRepositoryImpl::find(&mut transaction, tag_id).await {
        Ok(tag) => tag,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "Tag not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let tag = match TagRepositoryImpl::update(
        &mut transaction,
        &UpdateTag {
            id: tag.id,
            label: payload.label,
            tag_status: payload
                .tag_status
                .as_ref()
                .and_then(|s| TagStatus::from_str(s).ok()),
        },
    )
    .await
    {
        Ok(tag) => tag,
        Err(err) => {
            error!("{:#?}", err);
            if let Some(database_error) = err.as_database_error() {
                if let Some(constraint) = database_error.constraint() {
                    if constraint == "tags_label_key" {
                        return (StatusCode::BAD_REQUEST, "Tag already exists").into_response();
                    }
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(tag))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn delete_tag(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let tag_id = match params.tag_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let tag = match TagRepositoryImpl::find(&mut transaction, tag_id).await {
        Ok(tag) => tag,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "Tag not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let tag = match TagRepositoryImpl::delete(&mut transaction, tag.id).await {
        Ok(tag) => tag,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, format!("Deleted tag id: {}", tag.id)).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}
