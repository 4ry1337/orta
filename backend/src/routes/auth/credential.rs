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
use time::Duration;
use tracing::error;
use validator::Validate;

use crate::{
    application::AppState,
    configuration::CONFIG,
    models::{account_model::CreateAccount, user_model::CreateUser},
    repositories::{
        account_repository::{AccountRepository, AccountRepositoryImpl},
        user_repository::{UserRepository, UserRepositoryImpl},
    },
    utils::{
        fingerprint::{generate_fingerprint, verify_fingerprint_hash},
        jwt::{AccessToken, AccessTokenPayload, RefreshToken, RefreshTokenPayload, JWT},
        random_string::generate,
    },
};

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
    match AccountRepositoryImpl::create(
        &mut transaction,
        &CreateAccount {
            user_id: user.id,
            r#type: "credentails".to_string(),
            provider: "credentails".to_string(),
            provider_account_id: user.id.to_string(),
            expires_at: None,
            refresh_token: None,
            access_token: None,
            scope: None,
            token_type: None,
            id_token: None,
            session_state: None,
            password: Some(hashed_password),
            salt: Some(salt),
        },
    )
    .await
    {
        Ok(account) => account,
        Err(error) => {
            error!("unable to set password {}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    if let Err(err) = transaction.commit().await {
        error!("{:#?}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
    }

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

    let (fingerprint, fingerprint_hash) = match generate_fingerprint() {
        Ok((fingerprint, fingerprint_hash)) => (fingerprint, fingerprint_hash),
        Err(err) => {
            error!("fingerprint error {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to genereate tokens",
            )
                .into_response();
        }
    };

    let refresh_token = match RefreshToken::generate(RefreshTokenPayload {
        user_id: user.id,
        fingerprint: fingerprint_hash,
    }) {
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

    // let access_token_cookie: Cookie = Cookie::build((
    //     &CONFIG.cookie.access_token.name,
    //     format!("{}.{}", CONFIG.cookie.salt, access_token.clone(),),
    // ))
    // .http_only(true)
    // .same_site(SameSite::Lax)
    // .max_age(Duration::minutes(CONFIG.cookie.access_token.duration))
    // .into();

    let fingerprint_cookie: Cookie = Cookie::build((
        &CONFIG.cookie.fingerprint.name,
        format!("{}.{}", CONFIG.cookie.salt, fingerprint),
    ))
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookie.refresh_token.duration))
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
        .add(refresh_token_cookie)
        .add(fingerprint_cookie);

    (StatusCode::OK, cookies, access_token).into_response()
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
    let mut transaction = match appstate.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let user = match UserRepositoryImpl::find_by_email(&mut transaction, &payload.email).await {
        Ok(user) => user,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "User not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let account = match AccountRepositoryImpl::find_by_user(&mut transaction, user.id).await {
        Ok(acccount) => acccount,
        Err(err) => {
            error!("{}", err);
            if let sqlx::error::Error::RowNotFound = err {
                //TODO: better error response
                return (StatusCode::NOT_FOUND, "Password not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    if let Err(err) = transaction.commit().await {
        error!("{:#?}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
    }

    let password = match account.password {
        Some(password) => password,
        None => {
            error!("Credentials Account does not have password");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let salt = match account.salt {
        Some(salt) => salt,
        None => {
            error!("Credentials Account does not have salt");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match password.strip_suffix(&salt) {
        Some(password) => match bcrypt::verify(payload.password, password) {
            Ok(verified) => verified,
            Err(err) => {
                error!("{:#?}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
            }
        },
        None => return (StatusCode::UNAUTHORIZED, "Invalid credentails").into_response(),
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

    let (fingerprint, fingerprint_hash) = match generate_fingerprint() {
        Ok((fingerprint, fingerprint_hash)) => (fingerprint, fingerprint_hash),
        Err(err) => {
            error!("fingerprint error {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to genereate tokens",
            )
                .into_response();
        }
    };

    let refresh_token = match RefreshToken::generate(RefreshTokenPayload {
        user_id: user.id,
        fingerprint: fingerprint_hash,
    }) {
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

    // let access_token_cookie: Cookie = Cookie::build((
    //     &CONFIG.cookie.access_token.name,
    //     format!("{}.{}", CONFIG.cookie.salt, access_token.clone(),),
    // ))
    // .http_only(true)
    // .same_site(SameSite::Lax)
    // .max_age(Duration::minutes(CONFIG.cookie.access_token.duration))
    // .into();

    let fingerprint_cookie: Cookie = Cookie::build((
        &CONFIG.cookie.fingerprint.name,
        format!("{}.{}", CONFIG.cookie.salt, fingerprint),
    ))
    .http_only(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookie.fingerprint.duration))
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
        .add(refresh_token_cookie)
        .add(fingerprint_cookie);

    (StatusCode::OK, cookies, access_token).into_response()
}

// TODO activiaton links
