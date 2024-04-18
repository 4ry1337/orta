use axum::async_trait;
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    url::Url,
    AuthUrl, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope, StandardTokenResponse, TokenResponse,
    TokenUrl,
};
use reqwest::Client;
use secrecy::ExposeSecret;
use serde::Deserialize;

use crate::{
    configuration::CONFIG,
    models::account_model::{GithubUser, GoogleUser},
    utils::errors::Error,
};

#[derive(Clone)]
pub struct AuthService {
    pub github: GithubOAuthClient,
    pub google: GoogleOAuthClient,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            github: GithubOAuthClient::build(),
            google: GoogleOAuthClient::build(),
        }
    }
}

#[async_trait]
pub trait OAuthClient<T> {
    fn build() -> Self;
    fn authorization_url(&self) -> (PkceCodeVerifier, Url, CsrfToken);
    // async fn get_token_response(
    //     &self,
    //     code: &str,
    //     code_verifier: &str,
    // ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, Error>;
    async fn get_profile(
        &self,
        token_response: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Result<T, Error>;
}

#[derive(Clone)]
pub struct GoogleOAuthClient {
    pub client: BasicClient,
}

#[async_trait]
impl OAuthClient<GoogleUser> for GoogleOAuthClient {
    fn build() -> Self {
        let client_id = ClientId::new(CONFIG.auth.google.client_id.expose_secret().to_string());
        let client_secret =
            ClientSecret::new(CONFIG.auth.google.client_secret.expose_secret().to_string());
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Unable to create auth url");
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
            .expect("Unable to create token url");
        let redirect_url = RedirectUrl::new(format!(
            "{}/api/auth/ggogle/callback",
            CONFIG.application.host
        ))
        .expect("Unable to create redirect url");
        let revocation_url = RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .expect("Unable to create revocation url");
        Self {
            client: BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
                .set_redirect_uri(redirect_url)
                .set_revocation_uri(revocation_url),
        }
    }
    fn authorization_url(&self) -> (PkceCodeVerifier, Url, CsrfToken) {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let (authorize_url, csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ))
            .set_pkce_challenge(pkce_code_challenge)
            .url();
        (pkce_code_verifier, authorize_url, csrf_state)
    }

    // async fn get_token_response(
    //     &self,
    //     code: &str,
    //     code_verifier: &str,
    // ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, Error> {
    //     let code = AuthorizationCode::new(code.to_string());
    //     let pkce_code_verifier = PkceCodeVerifier::new(code_verifier.to_string());
    //
    //     self.client
    //         .exchange_code(code)
    //         .set_pkce_verifier(pkce_code_verifier)
    //         .request_async(async_http_client)
    //         .await
    //         .map_err(Error::RequestToken)
    // }

    async fn get_profile(
        &self,
        token_response: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Result<GoogleUser, Error> {
        let provider_response = Client::new()
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .header("User-Agent", "Rust") // An user agent is required for github
            .bearer_auth(token_response.access_token().secret())
            .send()
            .await
            .map_err(Error::Reqwest)?;

        provider_response
            .json::<GoogleUser>()
            .await
            .map_err(Error::Reqwest)
    }
}

#[derive(Clone)]
pub struct GithubOAuthClient {
    pub client: BasicClient,
}

#[derive(Default, Deserialize)]
struct GitHubEmail {
    email: String,
    primary: bool,
    // verified: bool,
    // visibility: String,
}

#[async_trait]
impl OAuthClient<GithubUser> for GithubOAuthClient {
    fn build() -> Self {
        let client_id = ClientId::new(CONFIG.auth.github.client_id.expose_secret().to_string());
        let client_secret =
            ClientSecret::new(CONFIG.auth.github.client_secret.expose_secret().to_string());
        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
            .expect("Unable to create auth url");
        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
            .expect("Unable to create token url");
        let redirect_url = RedirectUrl::new(format!(
            "{}/api/auth/github/callback",
            CONFIG.application.host
        ))
        .expect("Unable to create redirect url");
        Self {
            client: BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
                .set_redirect_uri(redirect_url),
        }
    }
    fn authorization_url(&self) -> (PkceCodeVerifier, Url, CsrfToken) {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let (authorize_url, csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read:user".to_string()))
            .add_scope(Scope::new("user:email".to_string()))
            .set_pkce_challenge(pkce_code_challenge)
            .url();
        (pkce_code_verifier, authorize_url, csrf_state)
    }
    async fn get_profile(
        &self,
        token_response: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Result<GithubUser, Error> {
        let provider_response = Client::new()
            .get("https://api.github.com/user")
            .header("User-Agent", "Rust") // An user agent is required for github
            .bearer_auth(token_response.access_token().secret())
            .send()
            .await
            .map_err(Error::Reqwest)?;

        let mut github_user = provider_response
            .json::<GithubUser>()
            .await
            .map_err(Error::Reqwest)?;

        if github_user.email.is_none() {
            let provider_response = Client::new()
                .get("https://api.github.com/user/emails")
                .header("User-Agent", "Rust") // An user agent is required for github
                .bearer_auth(token_response.access_token().secret())
                .send()
                .await
                .map_err(Error::Reqwest)?;

            let emails = provider_response
                .json::<Vec<GitHubEmail>>()
                .await
                .map_err(Error::Reqwest)?;

            let email = emails
                .iter()
                .find(|email| email.primary)
                .unwrap_or(emails.iter().next().unwrap())
                .email
                .to_owned();

            github_user.email = Some(email);
        }

        if github_user.name.is_none() {
            github_user.name = github_user.email.clone();
        }

        Ok(github_user)
    }
}
