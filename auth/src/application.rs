use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use shared::{
    authproto::auth_server::AuthServer,
    configuration::{DatabaseSettings, Settings},
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::transport::{server::Router, Server};
use tracing::info;

use crate::service::AuthService;

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

        let port = configuration.application.port;

        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

        let state = Arc::new(AppState { db: pool });

        let credential_auth_service = AuthService {
            state: state.clone(),
        };

        let server = Server::builder().add_service(AuthServer::new(credential_auth_service));

        info!("Finished user service build");

        Ok(Self {
            port,
            server,
            address,
        })
    }

    pub async fn run(self) -> Result<(), tonic::transport::Error> {
        self.server.serve(self.address).await
    }
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}
