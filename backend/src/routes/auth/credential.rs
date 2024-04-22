use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Form, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use cookie::SameSite;
use serde::Deserialize;
use shared::{
    configuration::CONFIG,
    models::prelude::*,
    repositories::prelude::*,
    utils::{
        jwt::{AccessToken, AccessTokenPayload, RefreshToken, RefreshTokenPayload, JWT},
        random_string::generate,
    },
};
use time::Duration;
use tracing::error;
use validator::Validate;

use crate::application::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/credentail/signup", post(signup))
        .route("/auth/credentail/signin", post(signin))
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
    State(appstate): State<Arc<AppState>>,
    Form(payload): Form<SignUpRequest>,
) -> Response {
    unimplemented!();
    // let fingerprint_cookie: Cookie = Cookie::build((
    //     &CONFIG.cookies.fingerprint.name,
    //     format!("{}.{}", CONFIG.cookies.salt, fingerprint),
    // ))
    // .http_only(true)
    // .same_site(SameSite::Lax)
    // .max_age(Duration::minutes(CONFIG.cookies.refresh_token.duration))
    // .into();
    //
    // let refresh_token_cookie: Cookie = Cookie::build((
    //     &CONFIG.cookies.refresh_token.name,
    //     format!("{}.{}", CONFIG.cookies.salt, refresh_token),
    // ))
    // .http_only(true)
    // .same_site(SameSite::Lax)
    // .max_age(Duration::minutes(CONFIG.cookies.refresh_token.duration))
    // .into();
    //
    // let cookies = CookieJar::new()
    //     .add(refresh_token_cookie)
    //     .add(fingerprint_cookie);
    //
    // (StatusCode::OK, cookies, access_token).into_response()
}

#[derive(Debug, Validate, Deserialize)]
pub struct SignInRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

pub async fn signin(
    State(appstate): State<Arc<AppState>>,
    Form(payload): Form<SignInRequest>,
) -> Response {
    unimplemented!();
    // let fingerprint_cookie: Cookie = Cookie::build((
    //     &CONFIG.cookies.fingerprint.name,
    //     format!("{}.{}", CONFIG.cookies.salt, fingerprint),
    // ))
    // .http_only(true)
    // .same_site(SameSite::Lax)
    // .max_age(Duration::minutes(CONFIG.cookies.fingerprint.duration))
    // .into();
    //
    // let refresh_token_cookie: Cookie = Cookie::build((
    //     &CONFIG.cookies.refresh_token.name,
    //     format!("{}.{}", CONFIG.cookies.salt, refresh_token),
    // ))
    // .http_only(true)
    // .same_site(SameSite::Lax)
    // .max_age(Duration::minutes(CONFIG.cookies.refresh_token.duration))
    // .into();
    //
    // let cookies = CookieJar::new()
    //     .add(refresh_token_cookie)
    //     .add(fingerprint_cookie);
    //
    // (StatusCode::OK, cookies, access_token).into_response()
}

// TODO activiaton links
