use axum::http::StatusCode;
use tonic::Code;

pub fn code_to_statudecode(code: Code) -> StatusCode {
    match code {
        Code::Ok => StatusCode::OK,
        Code::AlreadyExists => StatusCode::BAD_REQUEST,
        Code::InvalidArgument => StatusCode::BAD_REQUEST,
        Code::NotFound => StatusCode::NOT_FOUND,
        Code::Unimplemented => StatusCode::NOT_IMPLEMENTED,
        Code::Unauthenticated => StatusCode::UNAUTHORIZED,
        Code::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
