use std::sync::Arc;

use axum::Router;

use crate::application::AppState;

pub mod credential;
pub mod github;
pub mod google;

#[derive(Debug, serde::Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .merge(credential::router())
        .merge(github::router())
        .merge(google::router())
}

//TODO: should i add secure?
