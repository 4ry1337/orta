use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Extension, Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use cookie::SameSite;
use secrecy::ExposeSecret;
use serde::Deserialize;
use time::Duration;
use tracing::error;
use validator::Validate;

use crate::{
    application::AppState,
    configuration::CONFIG,
    models::user_model::{CreateUser, User},
    repositories::{
        password_repository::{PasswordRepository, PasswordRepositoryImpl},
        user_repository::{UserRepository, UserRepositoryImpl},
    },
    utils::{
        jwt::{AccessToken, AccessTokenPayload, RefreshToken, RefreshTokenPayload, JWT},
        random_string::generate,
    },
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
    Extension(user): Extension<User>,
    State(appstate): State<Arc<AppState>>,
    Json(payload): Json<SignUpRequest>,
) -> Response {
    let mut transaction = match appstate.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let user = match UserRepositoryImpl::create(
        &mut transaction,
        &CreateUser {
            username: payload.username,
            email: payload.email,
            image: None,
            email_verified: None,
        },
    )
    .await
    {
        Ok(user) => user,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "User not found").into_response();
            }
            if let Some(database_error) = err.as_database_error() {
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
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let salt = generate(6);
    let hashed_password = match bcrypt::hash(payload.password, 10) {
        Ok(password) => password + &salt,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match PasswordRepositoryImpl::create(&mut transaction, user.id, &hashed_password, &salt).await {
        Ok(password) => password,
        Err(error) => {
            error!("unable to set password {}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    if let Err(err) = transaction.commit().await {
        error!("{:#?}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
    }

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
                "Unable to generate tokens",
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
            error!("unable generate tokens:\n{}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to generate tokens",
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
