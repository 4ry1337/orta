use std::sync::Arc;

use shared::{
    comment::{
        comment_service_server::CommentService, CreateRequest, DeleteRequest, GetAllRequest,
        GetRequest, UpdateRequest,
    },
    common::{Comment, FullComment, FullComments, MessageResponse},
    models::comment_model::{CreateComment, UpdateComment},
    repositories::comment_repository::{CommentRepository, CommentRepositoryImpl},
};
use tonic::{Request, Response, Status};
use tracing::error;

use crate::{
    application::AppState,
    utils::{
        permissions::{is_owner, ContentType},
        split_cursor::parse_cursor,
    },
};

#[derive(Clone)]
pub struct CommentServiceImpl {
    pub state: Arc<AppState>,
}

#[tonic::async_trait]
impl CommentService for CommentServiceImpl {
    async fn get_all(
        &self,
        request: Request<GetAllRequest>,
    ) -> Result<Response<FullComments>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let mut id = None;
        let mut created_at = None;

        if let Some(cursor_str) = &input.cursor {
            (id, created_at) = match parse_cursor(cursor_str) {
                Ok(parsed) => parsed,
                Err(err) => {
                    error!("Parse error {}", err);
                    return Err(Status::invalid_argument("Invalid data"));
                }
            }
        };

        let comments = match CommentRepositoryImpl::find_all(
            &mut transaction,
            input.query.as_deref(),
            &input.target_id,
            input.r#type().into(),
            input.limit,
            id,
            created_at,
            input.by_user.as_deref(),
        )
        .await
        {
            Ok(comments) => comments,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let next_cursor = comments
            .iter()
            .nth(input.limit as usize - 1)
            .map(|item| format!("{}_{}", item.id, item.created_at.to_rfc3339()));

        let comments = comments
            .iter()
            .map(|comment| FullComment::from(comment))
            .collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(FullComments {
                comments,
                next_cursor,
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<FullComment>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let comment = match CommentRepositoryImpl::find(
            &mut transaction,
            &input.target_id,
            input.by_user.as_deref(),
        )
        .await
        {
            Ok(comment) => comment,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(FullComment::from(&comment))),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn create(&self, request: Request<CreateRequest>) -> Result<Response<Comment>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let comment = match CommentRepositoryImpl::create(
            &mut transaction,
            &CreateComment {
                user_id: input.user_id.to_owned(),
                target_id: input.target_id.to_owned(),
                content: input.content.to_owned(),
                r#type: input.r#type().into(),
            },
        )
        .await
        {
            Ok(comment) => comment,
            Err(err) => {
                error!("{:?}", err);
                if let Some(database_error) = err.as_database_error() {
                    if let Some(constraint) = database_error.constraint() {
                        if constraint == "comments_commenter_id_fkey" {
                            return Err(Status::not_found("User not found"));
                        }
                        // if constraint == "comments_article_id_fkey" {
                        //     return Err(Status::not_found("Article not found"));
                        // }
                    }
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(Comment::from(&comment))),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn update(&self, request: Request<UpdateRequest>) -> Result<Response<Comment>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        match is_owner(
            &mut transaction,
            ContentType::Comment,
            &input.user_id,
            &input.comment_id,
        )
        .await
        {
            Ok(is_owner) => {
                if !is_owner {
                    return Err(Status::permission_denied("Forbidden"));
                }
            }
            Err(err) => {
                error!("{:?}", err);

                // TODO check if works
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("Comment not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let comment = match CommentRepositoryImpl::update(
            &mut transaction,
            &UpdateComment {
                id: input.comment_id.to_owned(),
                content: input.content.to_owned(),
            },
        )
        .await
        {
            Ok(comment) => comment,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(Comment::from(&comment))),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        match is_owner(
            &mut transaction,
            ContentType::Comment,
            &input.user_id,
            &input.comment_id,
        )
        .await
        {
            Ok(is_owner) => {
                if !is_owner {
                    return Err(Status::permission_denied("Forbidden"));
                }
            }
            Err(err) => {
                error!("{:?}", err);

                // TODO check if works
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("Comment not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match CommentRepositoryImpl::delete(&mut transaction, &input.comment_id).await {
            Ok(comment) => comment,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(MessageResponse {
                message: format!("Delete comment"),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}
