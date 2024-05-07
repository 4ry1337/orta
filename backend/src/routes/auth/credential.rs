use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Extension, Form, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use cookie::SameSite;
use serde::Deserialize;
use shared::{
    auth_proto::{auth_service_client::AuthServiceClient, SigninRequest, SignupRequest},
    configuration::CONFIG,
};
use time::Duration;
use tonic::transport::Channel;
use tracing::error;
use validator::Validate;

use crate::{application::AppState, utils::mapper::code_to_statudecode};

pub fn router() -> Router<AppState> {
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

pub async fn signup(State(state): State<AppState>, Form(payload): Form<SignUpRequest>) -> Response {
    let res = match AuthServiceClient::new(state.auth_server.clone())
        .signup(SignupRequest {
            email: payload.email,
            password: payload.password,
            usermame: payload.username,
        })
        .await
    {
        Ok(res) => res.get_ref().to_owned(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    };

    let fingerprint_cookie: Cookie = Cookie::build((
        &CONFIG.cookies.fingerprint.name,
        format!("{}.{}", CONFIG.cookies.salt, res.fingerprint),
    ))
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookies.refresh_token.duration))
    .into();

    let refresh_token_cookie: Cookie = Cookie::build((
        &CONFIG.cookies.refresh_token.name,
        format!("{}.{}", CONFIG.cookies.salt, res.refresh_token),
    ))
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookies.refresh_token.duration))
    .into();

    let cookies = CookieJar::new()
        .add(refresh_token_cookie)
        .add(fingerprint_cookie);

    (StatusCode::OK, cookies, res.access_token).into_response()
}

#[derive(Debug, Validate, Deserialize)]
pub struct SignInRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

pub async fn signin(State(state): State<AppState>, Form(payload): Form<SignInRequest>) -> Response {
    let res = match AuthServiceClient::new(state.auth_server.clone())
        .signin(SigninRequest {
            email: payload.email,
            password: payload.password,
        })
        .await
    {
        Ok(res) => res.get_ref().to_owned(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            return (
                StatusCode::from_u16(err.to_http().status().as_u16())
                    .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR),
                message,
            )
                .into_response();
        }
    };
    let fingerprint_cookie: Cookie = Cookie::build((
        &CONFIG.cookies.fingerprint.name,
        format!("{}.{}", CONFIG.cookies.salt, res.fingerprint),
    ))
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookies.fingerprint.duration))
    .into();

    let refresh_token_cookie: Cookie = Cookie::build((
        &CONFIG.cookies.refresh_token.name,
        format!("{}.{}", CONFIG.cookies.salt, res.refresh_token),
    ))
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookies.refresh_token.duration))
    .into();

    let cookies = CookieJar::new()
        .add(refresh_token_cookie)
        .add(fingerprint_cookie);

    (StatusCode::OK, cookies, res.access_token).into_response()
}

// TODO activiaton links
