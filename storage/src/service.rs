use minio::s3::{
    args::{GetObjectArgs, PutObjectApiArgs},
    client::Client,
};
use shared::storage_proto::{
    storage_service_server::StorageService, RetriveRequest, RetriveResponse, StoreRequest,
    StoreResponse,
};
use tonic::{Request, Response, Status};
use tracing::{error, info};

#[derive(Clone)]
pub struct StorageServiceImpl {
    pub bucket_name: String,
    pub client: Client,
}

#[tonic::async_trait]
impl StorageService for StorageServiceImpl {
    async fn store(
        &self,
        request: Request<StoreRequest>,
    ) -> Result<Response<StoreResponse>, Status> {
        let input = request.get_ref();

        info!("Store Request {:?}", input.asset_name);

        let asset_name = format!(
            "{}_{}",
            chrono::Utc::now().format("%d-%m-%Y_%H:%M:%S"),
            &input.asset_name
        );

        match self
            .client
            .put_object_api(
                &mut PutObjectApiArgs::new(&self.bucket_name, &asset_name, &input.asset_data)
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

        info!("asset `{asset_name}` uploaded.");

        Ok(Response::new(StoreResponse { asset_name }))
    }

    async fn retrive(
        &self,
        request: Request<RetriveRequest>,
    ) -> Result<Response<RetriveResponse>, Status> {
        info!("Retrive Request");

        let input = request.get_ref();

        let response = match self
            .client
            .get_object_old(&GetObjectArgs::new(&self.bucket_name, &input.asset_name).unwrap())
            .await
        {
            Ok(res) => res,
            Err(err) => {
                error!(?err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        info!("asset `{}` retrived.", input.asset_name);

        let data = match response.bytes().await {
            Ok(data) => data.to_vec(),
            Err(err) => {
                error!(?err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        Ok(Response::new(RetriveResponse { data }))
    }
}
