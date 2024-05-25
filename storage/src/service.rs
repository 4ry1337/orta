use std::{
    io::{Cursor, Read},
    str::Bytes,
    sync::Arc,
};

use minio::s3::{
    args::{PutObjectApiArgs, PutObjectArgs},
    client::Client,
};
use shared::storage_proto::{
    storage_service_server::StorageService, Asset, RetriveRequest, StoreRequest,
};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use tracing::{error, info};

#[derive(Clone)]
pub struct StorageServiceImpl {
    pub bucket_name: String,
    pub client: Arc<Mutex<Client>>,
}

#[tonic::async_trait]
impl StorageService for StorageServiceImpl {
    async fn store(&self, request: Request<StoreRequest>) -> Result<Response<Asset>, Status> {
        let input = request.get_ref();

        info!("Store Request {:?}", input);

        let object_name = format!(
            "{}_{}",
            chrono::Utc::now().format("%d-%m-%Y_%H:%M:%S"),
            &input.asset_name
        );

        match self
            .client
            .lock()
            .await
            .put_object_api(
                &mut PutObjectApiArgs::new(&self.bucket_name, &object_name, &input.asset_data)
                    .unwrap(),
            )
            .await
        {
            Ok(put_object_response) => put_object_response,
            Err(err) => {
                error!(?err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        info!("asset `{object_name}` uploaded.");

        Ok(Response::new(Asset { asset_data: vec![] }))
    }
    //
    // async fn retrive(&self, request: Request<RetriveRequest>) -> Result<Response<Asset>, Status> {
    //     info!("Retrive Request");
    //
    //     let input = request.get_ref();
    //
    //     let response = match self
    //         .client
    //         .lock()
    //         .await
    //         .get_object(&GetObjectArgs::new(&self.bucket_name, &input.asset_name).unwrap())
    //         .await
    //     {
    //         Ok(res) => res,
    //         Err(err) => {
    //             error!(?err);
    //             return Err(Status::internal("Something went wrong"));
    //         }
    //     };
    //
    //     info!("asset `{}` retrived.", input.asset_name);
    //
    //     Ok(Response::new(Asset { asset_data: vec![] }))
    // }
}
