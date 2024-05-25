use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use minio::s3::{
    args::{BucketExistsArgs, MakeBucketArgs},
    client::{Client, ClientBuilder},
    creds::StaticProvider,
    http::BaseUrl,
};
use secrecy::ExposeSecret;
use shared::{
    configuration::{Settings, StorageSettings},
    storage_proto::storage_service_server::StorageServiceServer,
};
use tokio::sync::Mutex;
use tonic::transport::{server::Router, Server};
use tracing::info;

use crate::service::StorageServiceImpl;

pub struct Application {
    pub port: u16,
    pub server: Router,
    pub address: SocketAddr,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        info!("Building auth service");

        let client = get_minio_client(&configuration.storage).await;

        let port = configuration.storage_server.port;

        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

        let storage_service = StorageServiceImpl {
            client: Arc::new(Mutex::new(client)),
            bucket_name: configuration.storage.bucket_name,
        };

        let server = Server::builder().add_service(StorageServiceServer::new(storage_service));

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
