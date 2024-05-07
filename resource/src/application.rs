use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use shared::{
    configuration::{DatabaseSettings, Settings},
    resource_proto::{
        article_service_server::ArticleServiceServer, comment_service_server::CommentServiceServer,
        list_service_server::ListServiceServer, series_service_server::SeriesServiceServer,
        tag_service_server::TagServiceServer, user_service_server::UserServiceServer,
    },
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::transport::{server::Router, Server};
use tracing::info;

use crate::services::{
    article_service::ArticleServiceImpl, comment_service::CommentServiceImpl,
    list_service::ListServiceImpl, series_service::SeriesServiceImpl, tag_service::TagServiceImpl,
    user_service::UserServiceImpl,
};

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
        info!("Building resource service");

        let pool = get_connection_pool(&configuration.database).await;

        let port = configuration.resource_server.port;

        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

        let state = Arc::new(AppState { db: pool });

        let server = Server::builder()
            .add_service(UserServiceServer::new(UserServiceImpl {
                state: state.clone(),
            }))
            .add_service(ArticleServiceServer::new(ArticleServiceImpl {
                state: state.clone(),
            }))
            .add_service(ListServiceServer::new(ListServiceImpl {
                state: state.clone(),
            }))
            .add_service(SeriesServiceServer::new(SeriesServiceImpl {
                state: state.clone(),
            }))
            .add_service(CommentServiceServer::new(CommentServiceImpl {
                state: state.clone(),
            }))
            .add_service(TagServiceServer::new(TagServiceImpl {
                state: state.clone(),
            }));

        info!("Finished resource service build");

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
