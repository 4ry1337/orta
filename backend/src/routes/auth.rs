use std::sync::Arc;

use crate::{
    repository::user_repository::{CreateSession, CreateUser, UserRepository},
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use axum_core::response::IntoResponse;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct SigninRequest {
    pub email: String,
    pub password: String,
}

#[allow(non_snake_case)]
pub struct SigninResponse {
    pub email: String,
    pub emailVerified: Option<DateTime<Utc>>,
    pub id: String,
}

pub async fn signin(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SigninRequest>,
) -> impl IntoResponse {
    println!("{:?}", payload);
    let user = state.repository.user.get_user_by_email(payload.email).await;
    match user {
        Ok(user) => match user {
            Some(user) => match &user.password {
                Some(password) => {
                    let authentificated = bcrypt::verify(payload.password, password.as_str());
                    match authentificated {
                        Ok(auth) => {
                            if auth {
                                return (StatusCode::OK, Json(json!(user)));
                            };
                            (StatusCode::UNAUTHORIZED, Json(json!("Invalid credentials")))
                        }
                        Err(e) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!(e.to_string())),
                        ),
                    }
                }
                None => (StatusCode::UNAUTHORIZED, Json(json!("Invalid credentials"))),
            },
            None => (
                StatusCode::BAD_REQUEST,
                Json(json!("User with {payload.email} does not exist")),
            ),
        },
        Err(error) => (StatusCode::BAD_REQUEST, Json(json!(error.to_string()))),
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct SignupDetails {
    pub name: String,
    pub email: String,
    #[allow(non_snake_case)]
    pub emailVerified: Option<DateTime<Utc>>,
    pub password: Option<String>,
    pub image: Option<String>,
}

pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SignupDetails>,
) -> impl IntoResponse {
    let hashed_password: Option<String> = match payload.password {
        Some(pass) => {
            let hash = bcrypt::hash(pass, 10);
            match hash {
                Ok(hash) => Some(hash),
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!(e.to_string())),
                    );
                }
            }
        }
        _ => Option::None,
    };

    let user = state
        .repository
        .user
        .create(CreateUser {
            email: payload.email,
            name: payload.name,
            password: hashed_password,
            image: payload.image,
            emailVerified: payload.emailVerified,
        })
        .await;

    match user {
        Ok(_) => (StatusCode::CREATED, Json(json!("created new user"))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        ),
    }
}
