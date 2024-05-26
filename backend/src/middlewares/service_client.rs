use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use shared::configuration::CONFIG;
use tonic::transport::Channel;
use tracing::error;

pub async fn resource_service_middleware(mut req: Request, next: Next) -> Response {
    let resource_endpoint = Channel::from_shared(format!(
        "http://{}:{}",
        &CONFIG.resource_server.host, &CONFIG.resource_server.port
    ))
    .unwrap();

    match resource_endpoint.connect().await {
        Ok(client) => {
            req.extensions_mut().insert(client);
            return next.run(req).await;
        }
        Err(err) => {
            error!("Resource Server Unavailable {:?}", err);
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "Service is currently unavailable",
            )
                .into_response();
        }
    }
}

pub async fn auth_service_middleware(mut req: Request, next: Next) -> Response {
    let auth_endpoint = Channel::from_shared(format!(
        "http://{}:{}",
        &CONFIG.auth_server.host, &CONFIG.auth_server.port
    ))
    .unwrap();

    match auth_endpoint.connect().await {
        Ok(client) => {
            req.extensions_mut().insert(client);
            return next.run(req).await;
        }
        Err(err) => {
            error!("Auth Server Unavailable {:?}", err);
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "Service is currently unavailable",
            )
                .into_response();
        }
    }
}

pub async fn storage_service_middleware(mut req: Request, next: Next) -> Response {
    let storage_endpoint = Channel::from_shared(format!(
        "http://{}:{}",
        &CONFIG.storage_server.host, &CONFIG.storage_server.port
    ))
    .unwrap();

    match storage_endpoint.connect().await {
        Ok(client) => {
            req.extensions_mut().insert(client);
            return next.run(req).await;
        }
        Err(err) => {
            error!("Storage Server Unavailable {:?}", err);
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "Service is currently unavailable",
            )
                .into_response();
        }
    }
}
