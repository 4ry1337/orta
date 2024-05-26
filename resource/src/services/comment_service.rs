use std::sync::Arc;

use shared::{
    models::comment_model::{CreateComment, UpdateComment},
    repositories::comment_repository::{CommentRepository, CommentRepositoryImpl},
    resource_proto::{
        comment_service_server::CommentService, Comment, CreateCommentRequest,
        DeleteCommentRequest, DeleteCommentResponse, GetCommentsRequest, GetCommentsResponse,
        UpdateCommentRequest,
    },
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
    async fn create_comment(
        &self,
        request: Request<CreateCommentRequest>,
    ) -> Result<Response<Comment>, Status> {
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

    async fn get_comments(
        &self,
        request: Request<GetCommentsRequest>,
    ) -> Result<Response<GetCommentsResponse>, Status> {
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
            &input.target_id,
            input.r#type().into(),
            self.state.limit,
            id,
            created_at,
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
            .last()
            .map(|item| format!("{}_{}", item.id, item.created_at.to_string()));

        let comments = comments
            .iter()
            .map(|comment| Comment::from(comment))
            .collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(GetCommentsResponse {
                comments,
                next_cursor,
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn update_comment(
        &self,
        request: Request<UpdateCommentRequest>,
    ) -> Result<Response<Comment>, Status> {
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

    async fn delete_comment(
        &self,
        request: Request<DeleteCommentRequest>,
    ) -> Result<Response<DeleteCommentResponse>, Status> {
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
            Ok(_) => Ok(Response::new(DeleteCommentResponse {
                message: format!("Delete comment"),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}
