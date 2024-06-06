use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use chrono::Utc;
use cookie::SameSite;
use oauth2::{reqwest::async_http_client, AuthorizationCode, PkceCodeVerifier, TokenResponse};
use serde_json::json;
use time::Duration;
use tracing::{error, warn};

use crate::application::AppState;

use super::AuthRequest;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/github/signup", get(login))
        .route("/auth/github/callback", get(callback))
}

async fn login(State(appstate): State<Arc<AppState>>) -> Response {
    let (pkce_code_verifier, authorize_url, csrf_state) =
        appstate.services.auth.github.authorization_url();

    // let () = appstate.controllers.auth

    let csrf_cookie: Cookie = Cookie::build((
        &CONFIG.cookie.auth.csrf_state_name,
        format!("{}{}", CONFIG.cookie.salt, csrf_state.secret()),
    ))
    .http_only(true)
    .path("/")
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookie.auth.duration))
    .into();

    let code_verifier: Cookie = Cookie::build((
        &CONFIG.cookie.auth.code_verfier_name,
        format!("{}{}", CONFIG.cookie.salt, pkce_code_verifier.secret()),
    ))
    .http_only(true)
    .path("/")
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(CONFIG.cookie.auth.duration))
    .into();

    let cookies = CookieJar::new().add(csrf_cookie).add(code_verifier);

    (cookies, Redirect::to(authorize_url.as_str())).into_response()
}

async fn callback(
    cookies: CookieJar,
    State(appstate): State<Arc<AppState>>,
    Query(query): Query<AuthRequest>,
) -> Response {
    // let mut is_new_user = false;
    let code = query.code;
    let state = query.state;
    let stored_state = cookies.get(&format!(
        "{}.{}",
        CONFIG.cookie.salt, CONFIG.cookie.auth.csrf_state_name
    ));
    let stored_code_verifier = cookies.get(&format!(
        "{}.{}",
        CONFIG.cookie.salt, CONFIG.cookie.auth.code_verfier_name
    ));

    let (Some(csrf_state), Some(code_verifier)) = (stored_state, stored_code_verifier) else {
        warn!(name: "AUTH","csrf state and code are not specified");
        return (StatusCode::BAD_REQUEST).into_response();
    };
    if csrf_state.value() != state {
        warn!(name: "AUTH","csrf state is invalid");
        return (StatusCode::BAD_REQUEST).into_response();
    }

    let code = AuthorizationCode::new(code);
    let pkce_code_verifier = PkceCodeVerifier::new(code_verifier.value().to_owned());

    let token_response = match appstate
        .services
        .auth
        .github
        .client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await
    {
        Ok(token_response) => token_response,
        Err(error) => {
            error!("unable to get response from client: \n{}", error);
            return (StatusCode::BAD_REQUEST, "Something went wrong").into_response();
        }
    };

    // Get the Github user info
    let github_user = match appstate
        .services
        .auth
        .github
        .get_profile(&token_response)
        .await
    {
        Ok(github_user) => github_user,
        Err(error) => {
            error!("unable to get response from github:\n{}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let mut transaction = match appstate.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    // Add user session

    //TODO: understand this part https://github.com/nextauthjs/next-auth/blob/main/packages/core/src/lib/actions/callback/index.ts#L107
    //
    // find user from jwt token
    // then use it in userByAccount error

    //TODO: check if session token (access_token) exists then provide user using get by id (user) - signed user
    //
    //then find user by provider and provider_id
    //
    //if user by provider exists check it with a signed user if thier id is same return user
    //info, if not return error that account is associate with another user, then we can not
    //link them
    //
    //if signed user in none, return user info

    let user = match UserRepositoryImpl::find_by_account(
        &mut transaction,
        "github",
        &github_user.id.to_string(),
    )
    .await
    {
        Ok(user) => user,
        Err(_error) => {
            let username = match github_user.name {
                Some(name) => name,
                None => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response();
                }
            };
            let email = match github_user.email {
                Some(email) => email,
                None => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response();
                }
            };
            // if user from jwt exists { link accounts }
            // line 196
            let create_user = CreateUser {
                username,
                email,
                image: Some(github_user.avatar_url),
                email_verified: Some(Utc::now()),
            };
            // is_new_user = true;

            // If the user is not signed in and it looks like a new OAuth account then we
            // check there also isn't an user account already associated with the same
            // email address as the one in the OAuth profile.
            //
            // This step is often overlooked in OAuth implementations, but covers the following cases:
            //
            // 1. It makes it harder for someone to accidentally create two accounts.
            //    e.g. by signin in with email, then again with an oauth account connected to the same email.
            // 2. It makes it harder to hijack a user account using a 3rd party OAuth account.
            //    e.g. by creating an oauth account then changing the email address associated with it.
            //
            // It's quite common for services to automatically link accounts in this case, but it's
            // better practice to require the user to sign in *then* link accounts to be sure
            // someone is not exploiting a problem with a third party OAuth service.
            //
            // OAuth providers should require email address verification to prevent this, but in
            // practice that is not always the case; this helps protect against that.

            match UserRepositoryImpl::create(&mut transaction, &create_user).await {
                Ok(user) => user,
                Err(error) => {
                    error!("unable to create user:\n{}", error);
                    if let Some(database_error) = error.as_database_error() {
                        if let Some(constraint) = database_error.constraint() {
                            if constraint == "users_email_key" {
                                return (
                                    StatusCode::BAD_REQUEST,
                                    "Another account already exists with the same e-mail address",
                                )
                                    .into_response();
                            }
                        }
                    }
                    return (StatusCode::BAD_REQUEST, Json(json!(error.to_string())))
                        .into_response();
                }
            }
            // error!(name: "AUTH","unable to find user by account:\n{}", error);
        }
    };

    let expires_at: Option<i64> = match token_response.expires_in() {
        Some(expires_in) => Some((Utc::now() + expires_in).timestamp()),
        None => None,
    };

    let refresh_token: Option<String> = match token_response.refresh_token() {
        Some(refresh_token) => Some(refresh_token.secret().to_string()),
        None => None,
    };

    let scope: Option<String> = match token_response.scopes() {
        Some(scopes) => Some(
            scopes
                .iter()
                .map(|scope| scope.as_str().to_string())
                .collect(),
        ),
        None => None,
    };

    match AccountRepositoryImpl::create(
        &mut transaction,
        &CreateAccount {
            user_id: user.id,
            r#type: "oauth".to_string(),
            provider: "github".to_string(),
            provider_account_id: github_user.id.to_string(),
            refresh_token,
            access_token: Some(token_response.access_token().secret().to_string()),
            expires_at,
            token_type: Some(token_response.token_type().as_ref().to_string()),
            scope,
            id_token: None,
            session_state: None,
        },
    )
    .await
    {
        Ok(account) => account,
        Err(error) => {
            error!(name: "AUTH","unable to create account:\n{}", error);
            return (StatusCode::BAD_REQUEST, Json(json!(error.to_string()))).into_response();
        }
    };

    if let Err(err) = transaction.commit().await {
        error!("{:#?}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
    }
    // Remove code_verifier and csrf_state cookies
    let mut remove_csrf_cookie = Cookie::new(&CONFIG.cookie.auth.csrf_state_name, "");
    remove_csrf_cookie.set_path("/");
    remove_csrf_cookie.make_removal();

    let mut remove_code_verifier = Cookie::new(&CONFIG.cookie.auth.code_verfier_name, "");
    remove_code_verifier.set_path("/");
    remove_code_verifier.make_removal();

    //TODO: create session token (access_token) and return it in cookies
    // in next auth it is just adds fields to token
    //const token = await callbacks.jwt({
    //   token: defaultToken,
    //   user,
    //   account,
    //   profile: OAuthProfile,
    //   isNewUser,
    //   trigger: isNewUser ? "signUp" : "signIn",
    // })
    //
    // We will implement both tokens and persist refresh token

    let access_token = match AccessToken::generate(AccessTokenPayload {
        user_id: user.id,
        email: user.email,
        username: user.username,
        image: user.image,
        role: user.role,
    }) {
        Ok(access_token) => access_token,
        Err(error) => {
            error!("unable generate tokens: {:?}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response();
        }
    };

    let refresh_token = match RefreshToken::generate(RefreshTokenPayload {
        user_id: user.id,
        role: user.role,
        access_token: access_token.clone(),
    }) {
        Ok(refresh_token) => refresh_token,
        Err(error) => {
            error!("unable generate tokens: {:?}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
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
        .add(refresh_token_cookie)
        .add(remove_csrf_cookie)
        .add(remove_code_verifier);

    (StatusCode::OK, cookies).into_response()
}
