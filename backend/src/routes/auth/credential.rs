use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use cookie::SameSite;
use secrecy::ExposeSecret;
use serde::Deserialize;
use serde_json::json;
use time::Duration;
use tracing::error;
use validator::Validate;

use crate::{
    application::AppState,
    configuration::CONFIG,
    models::user_model::CreateUser,
    repositories::{password_repository::PasswordRepository, user_repository::UserRepository},
    utils::jwt::{AccessToken, AccessTokenPayload, RefreshToken, RefreshTokenPayload, JWT},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/auth/credentail/signup", post(signup))
}

#[derive(Debug, Validate, Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

pub async fn signup(
    cookies: CookieJar,
    State(appstate): State<Arc<AppState>>,
    Json(payload): Json<SignUpRequest>,
) -> Response {
    let create_user = CreateUser {
        username: payload.username,
        email: payload.email,
        image: None,
        email_verified: None,
    };

    let user = match appstate.repository.users.create(&create_user).await {
        Ok(user) => {
            match appstate
                .repository
                .users
                .password
                .create(user.id, &payload.password)
                .await
            {
                Ok(_) => user,
                Err(error) => {
                    error!(name: "AUTH", "unable to set password {}", error);
                    if let Err(_error) = appstate.repository.users.delete(user.id).await {
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

    let access_token = match AccessToken::generate(
        AccessTokenPayload {
            user_id: user.id,
            email: user.email,
            username: user.username,
            image: user.image,
            role: user.role,
        },
        &CONFIG.application.host,
        &CONFIG.auth.secret.expose_secret(),
    ) {
        Ok(access_token) => access_token,
        Err(error) => {
            error!(name: "AUTH","unable generate tokens:\n{}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response();
        }
    };

    let refresh_token = match RefreshToken::generate(
        RefreshTokenPayload {
            user_id: user.id,
            role: user.role,
            access_token: access_token.clone(),
        },
        &CONFIG.application.host,
        &CONFIG.auth.secret.expose_secret(),
    ) {
        Ok(refresh_token) => refresh_token,
        Err(error) => {
            error!(name: "AUTH","unable generate tokens:\n{}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response();
        }
    };

    let access_token_cookie: Cookie = Cookie::build((
        &CONFIG.cookie.access_token.name,
        format!("{}.{}", CONFIG.cookie.salt, access_token.clone(),),
    ))
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookie.access_token.duration))
    .into();

    let refresh_token_cookie: Cookie = Cookie::build((
        &CONFIG.cookie.refresh_token.name,
        format!("{}.{}", CONFIG.cookie.salt, refresh_token),
    ))
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookie.refresh_token.duration))
    .into();

    let cookies = CookieJar::new()
        .add(access_token_cookie)
        .add(refresh_token_cookie);

    (StatusCode::OK, cookies).into_response()
}
