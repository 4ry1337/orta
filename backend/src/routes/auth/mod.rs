use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_extra::extract::CookieJar;
use shared::{
    configuration::CONFIG,
    repositories::prelude::*,
    utils::jwt::{AccessToken, AccessTokenPayload, RefreshToken, JWT},
};
use tracing::error;

use crate::application::AppState;

pub mod credential;
// pub mod github;
// pub mod google;

#[derive(Debug, serde::Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .merge(credential::router())
        .route("/auth/refresh", get(refresh))
}

//TODO: should i add secure?

pub async fn refresh(cookies: CookieJar, State(state): State<Arc<AppState>>) -> Response {
    let refresh_token_with_prefix = match cookies.get(&CONFIG.cookies.refresh_token.name) {
        Some(refresh_token_cookie) => refresh_token_cookie.value(),
        None => return (StatusCode::UNAUTHORIZED, "Invalid credentails").into_response(),
    };
    let fingerprint_with_prefix = match cookies.get(&CONFIG.cookies.fingerprint.name) {
        Some(fingerprint_cookie) => fingerprint_cookie.value(),
        None => return (StatusCode::UNAUTHORIZED, "Invalid credentails").into_response(),
    };
    let refresh_token =
        match refresh_token_with_prefix.strip_prefix(&(CONFIG.cookies.salt.clone() + ".")) {
            Some(token) => token,
            None => return (StatusCode::BAD_REQUEST, "Invalid token").into_response(),
        };

    let fingerprint =
        match fingerprint_with_prefix.strip_prefix(&(CONFIG.cookies.salt.clone() + ".")) {
            Some(token) => token,
            None => return (StatusCode::BAD_REQUEST, "Invalid token").into_response(),
        };

    unimplemented!();
    //
    // (StatusCode::OK, access_token).into_response()
}
