use std::sync::Arc;

use shared::{
    models::prelude::*,
    repositories::prelude::*,
    resource_proto::{
        user_service_server::UserService, DeleteUserRequest, DeleteUserResponse, FollowUserRequest,
        FollowUserResponse, FullArticle, GetUserRequest, GetUserResponse, GetUsersRequest,
        GetUsersResponse, List, Series, Tag, UnfollowUserRequest, UnfollowUserResponse,
        UpdateUserRequest, UpdateUserResponse, User,
    },
    utils::params::Filter,
};
use tonic::{Request, Response, Status};
use tracing::error;

use crate::application::AppState;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub state: Arc<AppState>,
}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn get_users(
        &self,
        request: Request<GetUsersRequest>,
    ) -> Result<Response<GetUsersResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let total = match UserRepositoryImpl::total(&mut transaction).await {
            Ok(total) => match total {
                Some(total) => total,
                None => 0,
            },
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let users = match UserRepositoryImpl::find_all(
            &mut transaction,
            &Filter::from(&input.params),
        )
        .await
        {
            Ok(users) => users,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let users = users.iter().map(|user| User::from(user)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(GetUsersResponse { total, users })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let user =
            match UserRepositoryImpl::find_by_username(&mut transaction, &input.username).await {
                Ok(user) => user,
                Err(err) => {
                    error!("{:#?}", err);
                    if let sqlx::error::Error::RowNotFound = err {
                        return Err(Status::not_found("User not found"));
                    }
                    return Err(Status::internal("Something went wrong"));
                }
            };

        let articles = match ArticleRepositoryImpl::find_by_authors(
            &mut transaction,
            vec![user.username.clone()],
        )
        .await
        {
            Ok(articles) => articles,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let articles = articles
            .iter()
            .map(|article| FullArticle::from(article))
            .collect();

        let lists = match ListRepositoryImpl::find_by_user(&mut transaction, user.id).await {
            Ok(lists) => lists,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let lists = lists.iter().map(|list| List::from(list)).collect();

        let serieses = match SeriesRepositoryImpl::find_by_user(&mut transaction, user.id).await {
            Ok(serieses) => serieses,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let serieses = serieses.iter().map(|series| Series::from(series)).collect();

        let tags = match TagRepositoryImpl::find_by_user(&mut transaction, user.id).await {
            Ok(tags) => tags,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let interests = tags.iter().map(|tag| Tag::from(tag)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(GetUserResponse {
                user: Some(User::from(&user)),
                interests,
                articles,
                lists,
                serieses,
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };
        let input = request.get_ref();

        let user = match UserRepositoryImpl::find(&mut transaction, input.id).await {
            Ok(user) => user,
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("User not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let user = match UserRepositoryImpl::update(
            &mut transaction,
            &UpdateUser {
                id: user.id,
                username: input.username.clone(),
                image: user.image.clone(),
            },
        )
        .await
        {
            Ok(user) => user,
            Err(err) => {
                error!("{:#?}", err);
                if let Some(database_error) = err.as_database_error() {
                    if let Some(constraint) = database_error.constraint() {
                        if constraint == "users_email_key" {
                            return Err(Status::invalid_argument("Email is not available"));
                        }
                        if constraint == "users_username_key" {
                            return Err(Status::invalid_argument("Username is not available"));
                        }
                    }
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(UpdateUserResponse {
                message: format!("User updated: {}", user.id),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let user = match UserRepositoryImpl::find(&mut transaction, input.id).await {
            Ok(user) => user,
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("User not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let user = match UserRepositoryImpl::soft_delete(&mut transaction, user.id).await {
            Ok(user) => user,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(DeleteUserResponse {
                message: format!("Deleted user: {}", user.id),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn follow_user(
        &self,
        request: Request<FollowUserRequest>,
    ) -> Result<Response<FollowUserResponse>, Status> {
        Err(Status::unimplemented("Unimplemented"))
    }

    async fn unfollow_user(
        &self,
        request: Request<UnfollowUserRequest>,
    ) -> Result<Response<UnfollowUserResponse>, Status> {
        Err(Status::unimplemented("Unimplemented"))
    }
}
