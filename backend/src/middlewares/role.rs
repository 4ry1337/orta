use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

use shared::{models::enums::Role, utils::jwt::AccessTokenPayload};

pub async fn role_middleware(
    Extension(user): Extension<AccessTokenPayload>,
    State(state): State<Role>,
    req: Request,
    next: Next,
) -> Response {
    //TODO: better logic for roles
    if user.role == state || user.role == Role::Admin {
        return next.run(req).await;
    }
    (StatusCode::FORBIDDEN).into_response()
}
