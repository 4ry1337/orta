use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use tracing::error;

use crate::{
    application::AppState,
    configuration::CONFIG,
    repositories::user_repository::{UserRepository, UserRepositoryImpl},
    utils::jwt::{AccessToken, JWT},
};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Response {
    let token_with_prefix = match req.headers().typed_get::<Authorization<Bearer>>() {
        Some(token) => token,
        None => return (StatusCode::BAD_REQUEST, "No token").into_response(),
    };

    let token = match token_with_prefix
        .token()
        .strip_prefix(&(CONFIG.cookie.salt.clone() + "."))
    {
        Some(token) => token,
        None => return (StatusCode::BAD_REQUEST, "Invalid token").into_response(),
    };

    let token_payload = match AccessToken::validate(token) {
        Ok(token_payload) => token_payload,
        Err(error) => {
            // if let jsonwebtoken::errors::ErrorKind::ExpiredSignature = error {
            //
            // }
            error!("Unable to validate token: {:#?}", error);
            return (StatusCode::UNAUTHORIZED).into_response();
        }
    };

    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let user = match UserRepositoryImpl::find(&mut transaction, token_payload.payload.user_id).await
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

    if let Err(err) = transaction.commit().await {
        error!("{:#?}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
    };

    req.extensions_mut().insert(user);

    next.run(req).await
}
