use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use cookie::SameSite;
use serde_json::json;
use shared::{
    auth_proto::{auth_service_client::AuthServiceClient, RefreshRequest},
    configuration::CONFIG,
    utils::jwt::AccessTokenPayload,
};
use time::Duration;
use tracing::{error, info};

use crate::{
    application::AppState, middlewares::auth::auth_middleware, utils::mapper::code_to_statudecode,
};

pub mod credential;
// pub mod github;
// pub mod google;
//
// #[derive(Debug, serde::Deserialize)]
// pub struct AuthRequest {
//     code: String,
//     state: String,
// }

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(credential::router())
        .route("/auth/refresh", get(refresh))
        .route("/auth/signout", get(signout))
        .route(
            "/session",
            get(get_session).layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
}

//TODO: should i add secure?

pub async fn refresh(cookies: CookieJar, State(state): State<AppState>) -> Response {
    info!("Refresh token request");
    let refresh_token_with_prefix = match cookies.get(&CONFIG.cookies.refresh_token.name) {
        Some(refresh_token_cookie) => refresh_token_cookie.value(),
        None => {
            error!("Unable to get refresh token");
            return (StatusCode::UNAUTHORIZED, "Invalid credentails").into_response();
        }
    };
    let fingerprint_with_prefix = match cookies.get(&CONFIG.cookies.fingerprint.name) {
        Some(fingerprint_cookie) => fingerprint_cookie.value(),
        None => {
            error!("Unable to get fingerprint");
            return (StatusCode::UNAUTHORIZED, "Invalid credentails").into_response();
        }
    };
    let refresh_token =
        match refresh_token_with_prefix.strip_prefix(&(CONFIG.cookies.salt.clone() + ".")) {
            Some(token) => token.to_string(),
            None => {
                error!("Invalid refresh_token");
                return (StatusCode::BAD_REQUEST, "Invalid token").into_response();
            }
        };

    let fingerprint =
        match fingerprint_with_prefix.strip_prefix(&(CONFIG.cookies.salt.clone() + ".")) {
            Some(token) => token.to_string(),
            None => {
                error!("Invalid fingerprint");
                return (StatusCode::BAD_REQUEST, "Invalid token").into_response();
            }
        };

    match AuthServiceClient::new(state.auth_server.clone())
        .refresh(RefreshRequest {
            fingerprint,
            refresh_token,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (StatusCode::OK, res.access_token.to_owned()).into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn get_session(Extension(user): Extension<AccessTokenPayload>) -> Response {
    info!("Get session request");
    (StatusCode::OK, Json(json!(user))).into_response()
}

pub async fn signout() -> Response {
    info!("Signout request");

    let fingerprint_cookie: Cookie = Cookie::build((&CONFIG.cookies.fingerprint.name, ""))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::seconds(-1))
        .into();

    let refresh_token_cookie: Cookie = Cookie::build((&CONFIG.cookies.refresh_token.name, ""))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::seconds(-1))
        .into();

    let cookies = CookieJar::new()
        .add(refresh_token_cookie)
        .add(fingerprint_cookie);

    (StatusCode::OK, cookies).into_response()
}
