use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use shared::{
    configuration::{DatabaseSettings, Settings},
    resource_proto::user_service_server::UserServiceServer,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::transport::{server::Router, Server};
use tracing::info;

use crate::services::user_service::UserServiceImpl;

pub struct AppState {
    pub db: PgPool,
}

pub struct Application {
    pub port: u16,
    pub server: Router,
    pub address: SocketAddr,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        info!("Building user service");

        let pool = get_connection_pool(&configuration.database).await;

        let port = configuration.auth.port;

        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

        let state = Arc::new(AppState { db: pool });

        let user_service = UserServiceImpl {
            state: state.clone(),
        };

        let server = Server::builder().add_service(UserServiceServer::new(user_service));

        info!("Finished user service build");

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
