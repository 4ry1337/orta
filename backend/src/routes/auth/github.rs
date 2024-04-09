use std::sync::Arc;

use ::reqwest::Client;
use axum::{
    async_trait,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::Utc;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::Duration;

use crate::{
    config::{
        ACCESS_TOKEN_DURATION, ACCESS_TOKEN_NAME, COOKIE_AUTH_CODE_VERIFIER,
        COOKIE_AUTH_CSRF_STATE, COOKIE_SALT, GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, HOST,
        JWT_SECRET, REFRESH_TOKEN_DURATION, REFRESH_TOKEN_NAME,
    },
    models::{account_model::CreateAccount, user_model::CreateUser},
    repositories::{account_repository::AccountRepository, user_repository::UserRepository},
    utils::jwt::{AccessToken, AccessTokenPayload, RefreshToken, RefreshTokenPayload, JWT},
    AppState,
};
use tracing::{error, warn};

use super::{AuthRequest, OAuthClient};

pub struct GithubOAuthClient {
    pub client: BasicClient,
}

#[derive(Default, Serialize, Deserialize)]
struct GithubUser {
    id: u64,
    name: Option<String>,
    email: Option<String>,
    avatar_url: String,
}

#[derive(Default, Serialize, Deserialize)]
struct GitHubEmail {
    email: String,
    primary: bool,
    verified: bool,
    visibility: String,
}

#[async_trait]
impl OAuthClient for GithubOAuthClient {
    fn build(&self) -> Self {
        let client_id = ClientId::new(GOOGLE_CLIENT_ID.to_string());
        let client_secret = ClientSecret::new(GOOGLE_CLIENT_SECRET.to_string());
        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
            .expect("Unable to create auth url");
        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
            .expect("Unable to create token url");
        let redirect_url = RedirectUrl::new(format!("{}/api/auth/github/callback", HOST))
            .expect("Unable to create redirect url");
        Self {
            client: BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
                .set_redirect_uri(redirect_url),
        }
    }

    async fn login(&self) -> Response {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let (authorize_url, csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read:user".to_string()))
            .add_scope(Scope::new("user:email".to_string()))
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        let cookie_max_age = Duration::minutes(5);

        let csrf_cookie: Cookie =
            Cookie::build((COOKIE_AUTH_CSRF_STATE, csrf_state.secret().to_owned()))
                .http_only(true)
                .path("/")
                .same_site(SameSite::Lax)
                .max_age(cookie_max_age)
                .into();

        let code_verifier: Cookie = Cookie::build((
            COOKIE_AUTH_CODE_VERIFIER,
            pkce_code_verifier.secret().to_owned(),
        ))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Lax)
        .max_age(cookie_max_age)
        .into();

        let cookies = CookieJar::new().add(csrf_cookie).add(code_verifier);

        (cookies, Redirect::to(authorize_url.as_str())).into_response()
    }

    async fn callback(
        &self,
        State(appstate): State<Arc<AppState>>,
        Query(query): Query<AuthRequest>,
        cookies: CookieJar,
    ) -> Response {
        // let mut is_new_user = false;
        let code = query.code;
        let state = query.state;
        let stored_state = cookies.get(COOKIE_AUTH_CSRF_STATE);
        let stored_code_verifier = cookies.get(COOKIE_AUTH_CODE_VERIFIER);

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

        let token_response = match self
            .client
            .exchange_code(code)
            .set_pkce_verifier(pkce_code_verifier)
            .request_async(async_http_client)
            .await
        {
            Ok(token_response) => token_response,
            Err(error) => {
                error!(name: "AUTH","unable to get response from client: \n{}", error);
                return (StatusCode::BAD_REQUEST, "Something went wrong").into_response();
            }
        };

        // Get the Github user info
        let github_user = match Client::new()
            .get("https://api.github.com/user")
            .header("User-Agent", "Rust") // An user agent is required for github
            .bearer_auth(token_response.access_token().secret())
            .send()
            .await
        {
            Ok(github_response) => match github_response.json::<GithubUser>().await {
                Ok(github_user) => github_user,
                Err(error) => {
                    error!(name: "AUTH","unable to get response from github:\n{}", error);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response();
                }
            },
            Err(error) => {
                error!(name: "AUTH","{}", error);
                return (StatusCode::BAD_REQUEST, "Something went wrong").into_response();
            }
        };

        let email = match github_user.email {
            Some(email) => email,
            None => match Client::new()
                .get("https://api.github.com/user/emails")
                .header("User-Agent", "Rust") // An user agent is required for github
                .bearer_auth(token_response.access_token().secret())
                .send()
                .await
            {
                Ok(github_response) => match github_response.json::<Vec<GitHubEmail>>().await {
                    Ok(emails) => emails
                        .iter()
                        .find(|email| email.primary)
                        .unwrap_or(emails.iter().next().unwrap())
                        .email
                        .to_owned(),
                    Err(error) => {
                        error!(name: "AUTH","unable to parse response from github:\n{}", error);
                        return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                            .into_response();
                    }
                },
                Err(error) => {
                    error!(name: "AUTH","unable to get response from github:\n{}", error);
                    //in next-auth redirect to /signin
                    return (StatusCode::BAD_REQUEST, "Something went wrong").into_response();
                }
            },
        };

        // Add user session
        let account_id = github_user.id.to_string();

        let username = github_user.name.unwrap_or(email.clone());

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

        let user = match appstate
            .repository
            .users
            .find_by_account("github", &account_id)
            .await
        {
            Ok(user) => user,
            Err(error) => {
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

                match appstate.repository.users.create(&create_user).await {
                    Ok(user) => user,
                    Err(error) => {
                        error!(name: "OAUTH","unable to create user:\n{}", error);
                        if let Some(database_error) = error.as_database_error() {
                            if let Some(constraint) = database_error.constraint() {
                                if constraint == "users_email_key" {
                                    return (StatusCode::BAD_REQUEST, "Another account already exists with the same e-mail address")
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

        let new_account = CreateAccount {
            user_id: user.id,
            r#type: "oauth".to_string(),
            provider: "github".to_string(),
            provider_account_id: account_id,
            refresh_token,
            access_token: Some(token_response.access_token().secret().to_string()),
            expires_at,
            token_type: Some(token_response.token_type().as_ref().to_string()),
            scope,
            id_token: None,
            session_state: None,
        };

        let account = match appstate.repository.account.create(&new_account).await {
            Ok(account) => account,
            Err(error) => {
                error!(name: "AUTH","unable to create account:\n{}", error);
                return (StatusCode::BAD_REQUEST, Json(json!(error.to_string()))).into_response();
            }
        };

        // Remove code_verifier and csrf_state cookies
        let mut remove_csrf_cookie = Cookie::new(COOKIE_AUTH_CSRF_STATE, "");
        remove_csrf_cookie.set_path("/");
        remove_csrf_cookie.make_removal();

        let mut remove_code_verifier = Cookie::new(COOKIE_AUTH_CODE_VERIFIER, "");
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

        let access_token_payload = AccessTokenPayload {
            user_id: user.id,
            email: user.email,
            username: user.username,
            image: user.image,
            role: user.role,
        };

        let access_token = match AccessToken::generate("orta", access_token_payload, JWT_SECRET) {
            Ok(token) => token,
            Err(error) => {
                error!(name: "OAUTH","unable to generate access token:\n{}", error);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unable to generate tokens",
                )
                    .into_response();
            }
        };

        let access_token_cookie: Cookie = Cookie::build((
            format!("{}{}", COOKIE_SALT, ACCESS_TOKEN_NAME),
            access_token.clone(),
        ))
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::minutes(ACCESS_TOKEN_DURATION))
        .into();

        let refresh_token_payload = RefreshTokenPayload {
            user_id: user.id,
            role: user.role,
            access_token,
        };

        let refresh_token = match RefreshToken::generate("orta", refresh_token_payload, JWT_SECRET)
        {
            Ok(token) => token,
            Err(error) => {
                error!(name: "OAUTH","unable to generate refresh token:\n{}", error);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unable to generate tokens",
                )
                    .into_response();
            }
        };

        let refresh_token_cookie: Cookie = Cookie::build((
            format!("{}{}", COOKIE_SALT, REFRESH_TOKEN_NAME),
            refresh_token,
        ))
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::minutes(REFRESH_TOKEN_DURATION))
        .into();

        let cookies = CookieJar::new()
            .add(access_token_cookie)
            .add(refresh_token_cookie)
            .add(remove_csrf_cookie)
            .add(remove_code_verifier);

        (StatusCode::OK, cookies).into_response()
    }
}
