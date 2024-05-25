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
use minio::s3::{
    args::{BucketExistsArgs, MakeBucketArgs},
    creds::StaticProvider,
    http::BaseUrl,
    Client, ClientBuilder,
};
use secrecy::ExposeSecret;
use shared::configuration::{Settings, StorageSettings};
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
    pub storage: Client,
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

        let storage = get_minio_client(&configuration.storage).await;

        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

        let state = Arc::new(State {
            key: Key::generate(),
            storage,
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

pub async fn get_minio_client(configuration: &StorageSettings) -> Client {
    let base_url = "http://localhost:9000".parse::<BaseUrl>().unwrap();

    info!("Trying to connect to MinIO at: `{:?}`", base_url);

    let static_provider = StaticProvider::new(
        configuration.access_key.expose_secret(),
        configuration.secret_key.expose_secret(),
        None,
    );

    let client = ClientBuilder::new(base_url.clone())
        .provider(Some(Box::new(static_provider)))
        .build()
        .unwrap();

    let bucket_name = &configuration.bucket_name;

    let exists: bool = client
        .bucket_exists(&BucketExistsArgs::new(&bucket_name).unwrap())
        .await
        .unwrap();

    if !exists {
        client
            .make_bucket(&MakeBucketArgs::new(&bucket_name).unwrap())
            .await
            .unwrap();
    };

    client
}
