use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use amqprs::connection::{Connection, OpenConnectionArguments};
use secrecy::ExposeSecret;
use shared::{
    auth_proto::auth_service_server::AuthServiceServer,
    configuration::{DatabaseSettings, Settings},
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::transport::{server::Router, Server};
use tracing::info;

use crate::service::AuthServiceImpl;

pub struct AppState {
    pub db: PgPool,
    pub connection: Connection,
}

pub struct Application {
    pub port: u16,
    pub server: Router,
    pub address: SocketAddr,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        info!("Building auth service");

        let pool = get_connection_pool(&configuration.database).await;

        let port = configuration.auth_server.port;

        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

        let args = OpenConnectionArguments::new(
            &configuration.message_broker.hostname,
            configuration.message_broker.port,
            &configuration.message_broker.username,
            configuration.message_broker.password.expose_secret(),
        )
        .finish();

        let connection = Connection::open(&args).await.unwrap();

        let state = Arc::new(AppState {
            db: pool,
            connection,
        });

        let auth_service = AuthServiceImpl {
            state: state.clone(),
        };

        let server = Server::builder().add_service(AuthServiceServer::new(auth_service));

        info!("Finished auth service build");

        Ok(Self {
            port,
            server,
            address,
        })
    }

    pub async fn run(self) -> Result<(), tonic::transport::Error> {
        info!("Server is running on {}", self.port);
        self.server.serve(self.address).await
    }
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}
