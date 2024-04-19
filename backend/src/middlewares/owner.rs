use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

use crate::models::{enums::Role, user_model::User};

pub async fn content_owner_middleware(
    State(state): State<OwnerState>,
    Extension(user): Extension<User>,
    req: Request,
    next: Next,
) -> Response {
    next.run(req).await
}
