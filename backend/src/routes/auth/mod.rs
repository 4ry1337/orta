use std::sync::Arc;

use axum::{
    async_trait,
    extract::{Query, State},
    response::Response,
};
use axum_extra::extract::CookieJar;

use crate::AppState;

pub mod credential;
pub mod github;
pub mod google;

#[async_trait]
pub trait OAuthClient {
    fn build(&self) -> Self;
    async fn login(&self, State(state): State<Arc<AppState>>) -> Response;
    async fn callback(
        &self,
        State(state): State<Arc<AppState>>,
        Query(query): Query<AuthRequest>,
        cookies: CookieJar,
    ) -> Response;
}

#[derive(Debug, serde::Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}
