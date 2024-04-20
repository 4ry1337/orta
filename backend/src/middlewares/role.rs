use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

use crate::models::{enums::Role, user_model::User};

pub async fn role_middleware(
    Extension(user): Extension<User>,
    State(state): State<Role>,
    req: Request,
    next: Next,
) -> Response {
    //TODO: better logic for roles
    if user.role != state {
        return (StatusCode::FORBIDDEN).into_response();
    }
    next.run(req).await
}
