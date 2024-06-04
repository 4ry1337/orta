use std::sync::Arc;

use shared::{
    repositories::tag_repository::{TagRepository, TagRepositoryImpl},
    resource_proto::{tag_service_server::TagService, GetTagsRequest, GetTagsResponse, Tag},
};
use tonic::{Request, Response, Status};
use tracing::{error, info};

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
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get Tags Request {:?}", input);

        let tags = match TagRepositoryImpl::find_all(
            &mut transaction,
            input.query.as_deref(),
            input.limit,
            input.user_id.as_deref(),
            input.article_id.as_deref(),
            input.tag_status.map(|_| input.tag_status().into()),
            input.cursor.as_deref(),
        )
        .await
        {
            Ok(tags) => tags,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };
        let next_cursor = tags
            .iter()
            .nth(input.limit as usize - 1)
            .map(|tag| format!("{}", tag.slug));

        let tags = tags.iter().map(|tag| Tag::from(tag)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(GetTagsResponse { tags, next_cursor })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}
