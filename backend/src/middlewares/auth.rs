use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use secrecy::ExposeSecret;
use tracing::error;

use crate::{
    application::AppState,
    configuration::CONFIG,
    repositories::user_repository::UserRepository,
    utils::jwt::{AccessToken, JWT},
};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Response {
    let token = match req.headers().typed_get::<Authorization<Bearer>>() {
        Some(token) => token,
        None => return (StatusCode::BAD_REQUEST, "No token").into_response(),
    };

    let token_payload =
        match AccessToken::validate(token.token(), CONFIG.auth.secret.expose_secret()) {
            Ok(token_payload) => token_payload,
            Err(error) => {
                error!("Unable to validate token: {}", error);
                return (StatusCode::UNAUTHORIZED).into_response();
            }
        };

    let user = match state
        .repository
        .users
        .find_by_id(token_payload.user_id)
        .await
    {
        Ok(user) => user,
        Err(error) => {
            if let sqlx::error::Error::RowNotFound = error {
                return (StatusCode::NOT_FOUND, "User not found").into_response();
            }
            error!("Unable to get user: {}", error);
            return (StatusCode::UNAUTHORIZED).into_response();
        }
    };

    req.extensions_mut().insert(user);

    next.run(req).await
}
