use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use axum::{extract::FromRef, Router};
use axum_extra::extract::cookie::Key;
use shared::configuration::Settings;
use tokio::{net::TcpListener, signal};
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    timeout::{RequestBodyTimeoutLayer, TimeoutLayer},
};

#[derive(Clone)]
pub struct AppState {
    pub key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

pub struct Application {
    port: u16,
    listener: TcpListener,
    appstate: Arc<AppState>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let port = configuration.application.port;

        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

        let appstate = Arc::new(AppState {
            key: Key::generate(),
        });

        let listener = TcpListener::bind(&address).await?;

        Ok(Self {
            port,
            listener,
            // address,
            appstate,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_origin(["http://localhost:3000".parse().unwrap()]);

        let middleware = tower::ServiceBuilder::new()
            .layer(CompressionLayer::new().quality(tower_http::CompressionLevel::Fastest))
            .layer(RequestBodyTimeoutLayer::new(Duration::from_secs(30)))
            .layer(TimeoutLayer::new(Duration::from_secs(30)))
            .layer(CatchPanicLayer::new());

        axum::serve(
            self.listener,
            // routes::router(self.appstate.clone())
            Router::new()
                .layer(middleware)
                .layer(cors)
                .with_state(self.appstate),
        )
        .with_graceful_shutdown(shutdown_signal())
        .await
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
