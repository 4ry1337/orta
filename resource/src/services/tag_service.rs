use std::sync::Arc;

use shared::{
    repositories::tag_repository::{TagRepository, TagRepositoryImpl},
    resource_proto::{tag_service_server::TagService, GetTagsRequest, GetTagsResponse, Tag},
    utils::params::Filter,
};
use tonic::{Request, Response, Status};
use tracing::error;

use crate::application::AppState;

#[derive(Clone)]
pub struct TagServiceImpl {
    pub state: Arc<AppState>,
}

#[tonic::async_trait]
impl TagService for TagServiceImpl {
    async fn get_tags(
        &self,
        request: Request<GetTagsRequest>,
    ) -> Result<Response<GetTagsResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let tags = match TagRepositoryImpl::find_all(
            &mut transaction,
            input.tag_status.map(|_| input.tag_status().into()),
            &Filter::from(&input.params),
        )
        .await
        {
            Ok(tags) => tags,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let tags = tags.iter().map(|tag| Tag::from(tag)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(GetTagsResponse { tags })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}
