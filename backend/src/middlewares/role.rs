use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

use crate::models::{enums::Role, user_model::User};

pub async fn role_middleware(
    State(state): State<Role>,
    Extension(user): Extension<User>,
    req: Request,
    next: Next,
) -> Response {
    if user.role != state {
        return (StatusCode::FORBIDDEN).into_response();
    }
    next.run(req).await
}
