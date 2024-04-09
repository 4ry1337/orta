use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;

use crate::repositories::PgRepository;

#[derive(Clone)]
pub struct AppState {
    pub key: Key,
    pub repository: PgRepository,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

impl FromRef<AppState> for PgRepository {
    fn from_ref(state: &AppState) -> Self {
        state.repository.clone()
    }
}

pub const HOST: &str = dotenv!("HOST");
pub const DATABASE_URL: &str = dotenv!("DATABASE_URL");
pub const PORT: &str = dotenv!("PORT");
pub const JWT_SECRET: &str = dotenv!("JWT_SECRET");
pub const GITHUB_CLIENT_ID: &str = dotenv!("GITHUB_CLIENT_ID");
pub const GITHUB_CLIENT_SECRET: &str = dotenv!("GITHUB_CLIENT_SECRET");

pub const GOOGLE_CLIENT_ID: &str = dotenv!("GOOGLE_CLIENT_ID");
pub const GOOGLE_CLIENT_SECRET: &str = dotenv!("GOOGLE_CLIENT_SECRET");

pub const COOKIE_AUTH_CSRF_STATE: &str = "auth_csrf_state";
pub const COOKIE_AUTH_CODE_VERIFIER: &str = "auth_code_verifier";
//used when csrf and csrf verifier created
pub const COOKIE_AUTH_OAUTH_SIGNIN_AGE: i32 = 5;

pub const COOKIE_THEME: &str = "theme";
pub const COOKIE_SALT: &str = "__Secure.orta";
pub const ACCESS_TOKEN_NAME: &str = "access_token";
pub const ACCESS_TOKEN_DURATION: i64 = 5;
pub const REFRESH_TOKEN_NAME: &str = "refresh_token";
pub const REFRESH_TOKEN_DURATION: i64 = 60 * 24 * 30;
