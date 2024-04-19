use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::{net::TcpListener, signal};
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    timeout::{RequestBodyTimeoutLayer, TimeoutLayer},
};

use crate::{
    configuration::{DatabaseSettings, Settings},
    routes,
    services::Services,
};

#[derive(Clone)]
pub struct AppState {
    pub key: Key,
    pub db: PgPool,
    pub services: Services,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

impl FromRef<AppState> for Services {
    fn from_ref(state: &AppState) -> Self {
        state.services.clone()
    }
}

pub struct Application {
    port: u16,
    listener: TcpListener,
    address: SocketAddr,
    appstate: Arc<AppState>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        dotenv().ok();

        let pool = get_connection_pool(&configuration.database).await;
        // let email_client = configuration.email_client.client();

        let port = configuration.application.port;

        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

        let appstate = Arc::new(AppState {
            key: Key::generate(),
            db: pool,
            services: Services::new(),
        });

        let listener = TcpListener::bind(&address).await?;

        Ok(Self {
            port,
            listener,
            address,
            appstate,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
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

        //TODO: refactor to routes
        // let app = routes::router()
        //     .layer(middleware)
        //     .layer(cors)
        //     .with_state(appstate);

        axum::serve(
            self.listener,
            routes::router(self.appstate.clone())
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

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    // let pool = match PgPoolOptions::new()
    //     .max_connections(10)
    //     .connect(DATABASE_URL)
    //     .await
    // {
    //     Ok(pool) => {
    //         info!("Connection to the database is successful!");
    //         pool
    //     }
    //     Err(err) => {
    //         error!("Failed to connect to the database: {:?}", err);
    //         panic!("Failed to connect to the database");
    //     }
    // };

    let pool = PgPoolOptions::new().connect_lazy_with(configuration.with_db());

    // sqlx::migrate!()
    //     .run(&pool)
    //     .await
    //     .expect("Failed migrations");

    pool
}
