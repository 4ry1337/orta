use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn health_checker() -> Response {
    (StatusCode::OK, "Orta is running").into_response()
}
