use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use tracing::error;

use crate::utils::jwt::{AccessToken, JWT};

pub async fn auth_middleware(mut req: Request, next: Next) -> Response {
    let token = match req.headers().typed_get::<Authorization<Bearer>>() {
        Some(token) => token,
        None => return (StatusCode::BAD_REQUEST, "No token").into_response(),
    };

    let access_token_payload = match AccessToken::validate(token.token()) {
        Ok(token_payload) => token_payload,
        Err(error) => {
            error!("Unable to validate token: {:#?}", error);
            return (StatusCode::UNAUTHORIZED, "Verification failed").into_response();
        }
    };

    req.extensions_mut().insert(access_token_payload.payload);

    next.run(req).await
}
