use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_extra::extract::CookieJar;
use tracing::error;

use crate::{
    application::AppState,
    configuration::CONFIG,
    repositories::user_repository::{UserRepository, UserRepositoryImpl},
    utils::{
        fingerprint::verify_fingerprint_hash,
        jwt::{AccessToken, AccessTokenPayload, RefreshToken, JWT},
    },
};

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
    // .merge(github::router())
    // .merge(google::router())
}

//TODO: should i add secure?

pub async fn refresh(cookies: CookieJar, State(state): State<Arc<AppState>>) -> Response {
    let refresh_token_with_prefix = match cookies.get(&CONFIG.cookie.refresh_token.name) {
        Some(refresh_token_cookie) => refresh_token_cookie.value(),
        None => return (StatusCode::UNAUTHORIZED, "Invalid credentails").into_response(),
    };
    let fingerprint_with_prefix = match cookies.get(&CONFIG.cookie.fingerprint.name) {
        Some(fingerprint_cookie) => fingerprint_cookie.value(),
        None => return (StatusCode::UNAUTHORIZED, "Invalid credentails").into_response(),
    };
    let refresh_token =
        match refresh_token_with_prefix.strip_prefix(&(CONFIG.cookie.salt.clone() + ".")) {
            Some(token) => token,
            None => return (StatusCode::BAD_REQUEST, "Invalid token").into_response(),
        };

    let fingerprint =
        match fingerprint_with_prefix.strip_prefix(&(CONFIG.cookie.salt.clone() + ".")) {
            Some(token) => token,
            None => return (StatusCode::BAD_REQUEST, "Invalid token").into_response(),
        };

    let refresh_token_payload = match RefreshToken::validate(refresh_token) {
        Ok(token_payload) => token_payload,
        Err(error) => {
            error!("Unable to validate token: {:#?}", error);
            return (StatusCode::UNAUTHORIZED, "Verification failed").into_response();
        }
    };

    match verify_fingerprint_hash(&fingerprint, &refresh_token_payload.payload.fingerprint) {
        Ok(verified) => {
            if !verified {
                return (StatusCode::UNAUTHORIZED, "Verification failed").into_response();
            }
        }
        Err(err) => {
            error!("Verification failed: {:#?}", err);
            return (StatusCode::UNAUTHORIZED, "Verification failed").into_response();
        }
    }

    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let user =
        match UserRepositoryImpl::find(&mut transaction, refresh_token_payload.payload.user_id)
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

    if let Err(err) = transaction.commit().await {
        error!("{:#?}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
    };

    let access_token = match AccessToken::generate(AccessTokenPayload {
        user_id: user.id,
        email: user.email,
        username: user.username,
        image: user.image,
        role: user.role,
    }) {
        Ok(access_token) => access_token,
        Err(error) => {
            error!(name: "AUTH","unable generate tokens:\n{}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to generate tokens",
            )
                .into_response();
        }
    };

    (StatusCode::OK, access_token).into_response()
}
