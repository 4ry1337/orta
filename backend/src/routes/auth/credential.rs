use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use cookie::SameSite;
use serde::Deserialize;
use serde_json::json;
use time::Duration;
use tracing::error;
use validator::Validate;

use crate::{
    config::{
        ACCESS_TOKEN_DURATION, ACCESS_TOKEN_NAME, COOKIE_SALT, JWT_SECRET, REFRESH_TOKEN_DURATION,
        REFRESH_TOKEN_NAME,
    },
    models::user_model::CreateUser,
    repositories::{password_repository::PasswordRepository, user_repository::UserRepository},
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
    cookies: CookieJar,
) -> Response {
    let create_user = CreateUser {
        username: payload.username,
        email: payload.email,
        image: None,
        email_verified: None,
    };

    let user = match state.repository.users.create(&create_user).await {
        Ok(user) => {
            match state
                .repository
                .users
                .password
                .create(user.id, &payload.password)
                .await
            {
                Ok(_) => user,
                Err(error) => {
                    error!(name: "AUTH", "unable to set password {}", error);
                    if let Err(_error) = state.repository.users.delete(user.id).await {
                        return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                            .into_response();
                    }
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response();
                }
            }
        }
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

    let access_token = match AccessToken::generate("orta", access_token_payload, JWT_SECRET) {
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

    let access_token_cookie: Cookie = Cookie::build((
        format!("{}{}", COOKIE_SALT, ACCESS_TOKEN_NAME),
        access_token.clone(),
    ))
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(ACCESS_TOKEN_DURATION))
    .into();

    let refresh_token_payload = RefreshTokenPayload {
        user_id: user.id,
        role: user.role,
        access_token,
    };

    let refresh_token = match RefreshToken::generate("orta", refresh_token_payload, JWT_SECRET) {
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

    let refresh_token_cookie: Cookie = Cookie::build((
        format!("{}{}", COOKIE_SALT, REFRESH_TOKEN_NAME),
        refresh_token,
    ))
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(REFRESH_TOKEN_DURATION))
    .into();

    let cookies = CookieJar::new()
        .add(access_token_cookie)
        .add(refresh_token_cookie);

    (StatusCode::OK, cookies).into_response()
}
