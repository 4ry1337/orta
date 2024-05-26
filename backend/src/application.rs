use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use axum::{
    extract::{DefaultBodyLimit, FromRef},
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
};
use axum_extra::extract::cookie::Key;
use shared::configuration::Settings;
use tokio::{net::TcpListener, signal};
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::CorsLayer,
    timeout::{RequestBodyTimeoutLayer, TimeoutLayer},
    trace::TraceLayer,
};
use tracing::info;

use crate::routes;

#[derive(Clone)]
pub struct State {
    pub key: Key,
}

impl FromRef<State> for Key {
    fn from_ref(state: &State) -> Self {
        state.key.clone()
    }
}

pub type AppState = Arc<State>;

pub struct Application {
    port: u16,
    listener: TcpListener,
    state: AppState,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        info!("Building api gateway service");

        let port = configuration.api_server.port;

        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

        let state = Arc::new(State {
            key: Key::generate(),
        });

        let listener = TcpListener::bind(&address).await?;

        info!("Finished api gateway service build");

        Ok(Self {
            port,
            listener,
            // address,
            state,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        let cors = CorsLayer::new()
            .allow_origin("http://localhost:4000".parse::<HeaderValue>().unwrap())
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::PUT,
                Method::DELETE,
            ])
            .allow_credentials(true)
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

        let middleware = tower::ServiceBuilder::new()
            .layer(CompressionLayer::new().quality(tower_http::CompressionLevel::Fastest))
            .layer(RequestBodyTimeoutLayer::new(Duration::from_secs(30)))
            .layer(TimeoutLayer::new(Duration::from_secs(30)))
            .layer(CatchPanicLayer::new());

        info!("Server is running on {}", self.port);

        axum::serve(
            self.listener,
            routes::router(self.state.clone())
                .layer(DefaultBodyLimit::max(1024 * 1024 * 50))
                .layer(middleware)
                .layer(cors)
                .layer(TraceLayer::new_for_http())
                .with_state(self.state),
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
