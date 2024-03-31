use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{
    models::user_model::CreateUser,
    repositories::user_repository::UserRepository,
    utils::jwt::{AccessToken, AccessTokenPayload, RefreshToken, RefreshTokenPayload, JWT},
    AppState,
};

#[derive(Debug, Validate, Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SignUpRequest>,
    jar: CookieJar,
) -> Response {
    let hashed_password = match bcrypt::hash(payload.password, 10) {
        Ok(pass) => pass,
        Err(_error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error cant hash the password",
            )
                .into_response();
        }
    };

    let create_user = CreateUser {
        username: payload.username,
        email: payload.email,
        image: None,
        email_verified: None,
        password: Some(hashed_password),
    };

    let user = match state.repository.users.create(&create_user).await {
        Ok(user) => user,
        Err(error) => {
            if let Some(database_error) = error.as_database_error() {
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
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response();
        }
    };

    let access_token_payload = AccessTokenPayload {
        user_id: user.id,
        email: user.email,
        username: user.username,
        image: user.image,
        role: user.role,
    };

    let access_token =
        match AccessToken::generate("orta", access_token_payload, &state.config.jwt_secret) {
            Ok(token) => token,
            Err(error) => {
                println!("{:?}", error);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unable to generate tokens",
                )
                    .into_response();
            }
        };

    let refresh_token_payload = RefreshTokenPayload {
        user_id: user.id,
        role: user.role,
        access_token: access_token.clone(),
    };
    let refresh_token =
        match RefreshToken::generate("orta", refresh_token_payload, &state.config.jwt_secret) {
            Ok(token) => token,
            Err(error) => {
                println!("{:?}", error);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unable to generate tokens",
                )
                    .into_response();
            }
        };

    (
        StatusCode::OK,
        jar.add(Cookie::new("access_token", access_token))
            .add(Cookie::new("refresh_token", refresh_token)),
    )
        .into_response()
}
