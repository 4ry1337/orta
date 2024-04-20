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
use tracing::error;

use crate::{
    application::AppState,
    models::user_model::{CreateUser, UpdateUser},
    repositories::user_repository::{UserRepository, UserRepositoryImpl},
    utils::params::PathParams,
};

pub async fn get_users(State(state): State<Arc<AppState>>) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let users = match UserRepositoryImpl::find_all(&mut transaction).await {
        Ok(users) => users,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(users))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let user_id = match params.user_id {
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

    let user = match UserRepositoryImpl::find(&mut transaction, user_id).await {
        Ok(user) => user,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "User not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(user))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
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
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let user = match UserRepositoryImpl::create(
        &mut transaction,
        &CreateUser {
            username: payload.username,
            email: payload.email,
            image: payload.image,
            email_verified: payload.email_verified,
        },
    )
    .await
    {
        Ok(user) => user,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "User not found").into_response();
            }
            if let Some(database_error) = err.as_database_error() {
                if let Some(constraint) = database_error.constraint() {
                    if constraint == "users_email_key" {
                        return (StatusCode::BAD_REQUEST, "Email is not available").into_response();
                    }
                    if constraint == "users_username_key" {
                        return (StatusCode::BAD_REQUEST, "Username is not available")
                            .into_response();
                    }
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match transaction.commit().await {
        Ok(_) => (StatusCode::CREATED, Json(json!(user))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
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
    let user_id = match params.user_id {
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

    let user = match UserRepositoryImpl::find(&mut transaction, user_id).await {
        Ok(user) => user,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "User not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let user = match UserRepositoryImpl::update(
        &mut transaction,
        &UpdateUser {
            id: user.id,
            username: payload.username,
            image: payload.image,
        },
    )
    .await
    {
        Ok(user) => user,
        Err(err) => {
            error!("{:#?}", err);
            if let Some(database_error) = err.as_database_error() {
                if let Some(constraint) = database_error.constraint() {
                    if constraint == "users_email_key" {
                        return (StatusCode::BAD_REQUEST, "Email is not available").into_response();
                    }
                    if constraint == "users_username_key" {
                        return (StatusCode::BAD_REQUEST, "Username is not available")
                            .into_response();
                    }
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(user))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let user_id = match params.user_id {
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

    let user = match UserRepositoryImpl::find(&mut transaction, user_id).await {
        Ok(user) => user,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "User not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let user = match UserRepositoryImpl::delete(&mut transaction, user.id).await {
        Ok(user) => user,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response()
        }
    };

    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, format!("Deleted user: {}", user.id)).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}
