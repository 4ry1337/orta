use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Extension, Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use cookie::SameSite;
use serde::Deserialize;
use shared::{
    auth_proto::{
        auth_service_client::AuthServiceClient, SigninRequest, SignupRequest, VerifyEmailRequest,
    },
    configuration::CONFIG,
};
use time::Duration;
use tonic::transport::Channel;
use tracing::{error, info};
use validator::Validate;

use crate::{application::AppState, utils::mapper::code_to_statudecode};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/credential/signup", post(signup))
        .route("/auth/credential/signin", post(signin))
        .route("/auth/credential/verify", post(signin))
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
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Json(payload): Json<SignUpRequest>,
) -> Response {
    info!("Signup request");
    match AuthServiceClient::new(channel)
        .signup(SignupRequest {
            email: payload.email,
            password: payload.password,
            usermame: payload.username,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

#[derive(Debug, Validate, Deserialize)]
pub struct SignInRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

pub async fn signin(
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Json(payload): Json<SignInRequest>,
) -> Response {
    info!("Signin request");
    let res = match AuthServiceClient::new(channel)
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
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    };
    let fingerprint_cookie: Cookie = Cookie::build((
        &CONFIG.cookies.fingerprint.name,
        format!("{}.{}", CONFIG.cookies.salt, res.fingerprint),
    ))
    .path("/")
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookies.fingerprint.duration))
    .into();

    let refresh_token_cookie: Cookie = Cookie::build((
        &CONFIG.cookies.refresh_token.name,
        format!("{}.{}", CONFIG.cookies.salt, res.refresh_token),
    ))
    .path("/")
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookies.refresh_token.duration))
    .into();

    let cookies = CookieJar::new()
        .add(refresh_token_cookie)
        .add(fingerprint_cookie);

    (StatusCode::OK, cookies, res.access_token).into_response()
}

#[derive(Debug, Deserialize)]
pub struct VerifyQueryParams {
    token: String,
}

pub async fn verify(
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Query(query): Query<VerifyQueryParams>,
) -> Response {
    info!("Verify request");
    let res = match AuthServiceClient::new(channel)
        .verify_email(VerifyEmailRequest { token: query.token })
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
    .path("/")
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookies.fingerprint.duration))
    .into();

    let refresh_token_cookie: Cookie = Cookie::build((
        &CONFIG.cookies.refresh_token.name,
        format!("{}.{}", CONFIG.cookies.salt, res.refresh_token),
    ))
    .path("/")
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookies.refresh_token.duration))
    .into();

    let cookies = CookieJar::new()
        .add(refresh_token_cookie)
        .add(fingerprint_cookie);

    (StatusCode::OK, cookies, res.access_token).into_response()
}
