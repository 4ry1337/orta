use std::sync::Arc;

use chrono::{DateTime, Utc};
use shared::{
    models::user_model::UpdateUser,
    repositories::user_repository::{UserRepository, UserRepositoryImpl},
    resource_proto::{
        user_service_server::UserService, DeleteUserRequest, DeleteUserResponse, FollowUserRequest,
        FollowUserResponse, FollowersRequest, FollowersResponse, FollowingRequest,
        FollowingResponse, FullUser, GetUserRequest, GetUsersRequest, GetUsersResponse,
        UnfollowUserRequest, UnfollowUserResponse, UpdateUserRequest, User,
    },
};
use tonic::{Request, Response, Status};
use tracing::{error, info};

use crate::{application::AppState, utils::split_cursor::parse_cursor};

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
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get Users Request {:?}", input);

        let mut id: Option<&str> = None;
        let mut created_at: Option<DateTime<Utc>> = None;

        if let Some(cursor_str) = &input.cursor {
            (id, created_at) = match parse_cursor(cursor_str) {
                Ok(parsed) => parsed,
                Err(err) => {
                    error!("Parse error {}", err);
                    return Err(Status::invalid_argument("Invalid data"));
                }
            }
        };

        let users = match UserRepositoryImpl::find_all(
            &mut transaction,
            input.query.as_deref(),
            input.limit,
            id,
            created_at,
            input.by_user.as_deref(),
        )
        .await
        {
            Ok(users) => users,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let next_cursor = users
            .iter()
            .nth(input.limit as usize - 1)
            .map(|item| format!("{}_{}", item.id, item.created_at.to_rfc3339()));

        let users = users.iter().map(|user| FullUser::from(user)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(GetUsersResponse { users, next_cursor })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<FullUser>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get User Request {:?}", input);

        let user = match UserRepositoryImpl::find_by_username(
            &mut transaction,
            &input.username,
            input.by_user.as_deref(),
        )
        .await
        {
            Ok(user) => user,
            Err(err) => {
                error!("{:?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("User not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(FullUser::from(&user))),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };
        let input = request.get_ref();

        info!("Update User Request {:?}", input);

        let user = match UserRepositoryImpl::find(&mut transaction, &input.id).await {
            Ok(user) => user,
            Err(err) => {
                error!("{:?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("User not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let user = match UserRepositoryImpl::update(
            &mut transaction,
            &UpdateUser {
                id: user.id.to_owned(),
                username: input.username.to_owned(),
                image: user.image.to_owned(),
            },
        )
        .await
        {
            Ok(user) => user,
            Err(err) => {
                error!("{:?}", err);
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
            Ok(_) => Ok(Response::new(User::from(&user))),
            Err(err) => {
                error!("{:?}", err);
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
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Soft Delete User Request {:?}", input);

        let user = match UserRepositoryImpl::find(&mut transaction, &input.id).await {
            Ok(user) => user,
            Err(err) => {
                error!("{:?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("User not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let user = match UserRepositoryImpl::soft_delete(&mut transaction, &user.id).await {
            Ok(user) => user,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(DeleteUserResponse {
                message: format!("Deleted user: {}", user.id),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn follow_user(
        &self,
        request: Request<FollowUserRequest>,
    ) -> Result<Response<FollowUserResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Follow User Request {:?}", input);

        let user =
            match UserRepositoryImpl::follow(&mut transaction, &input.user_id, &input.target_id)
                .await
            {
                Ok(user) => user,
                Err(err) => {
                    error!("{:?}", err);
                    if let sqlx::error::Error::RowNotFound = err {
                        return Err(Status::not_found("User not found"));
                    }
                    return Err(Status::internal("Something went wrong"));
                }
            };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(FollowUserResponse {
                message: format!("User {} followed {}", user.0, user.1),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn unfollow_user(
        &self,
        request: Request<UnfollowUserRequest>,
    ) -> Result<Response<UnfollowUserResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Follow User Request {:?}", input);

        let user =
            match UserRepositoryImpl::unfollow(&mut transaction, &input.user_id, &input.target_id)
                .await
            {
                Ok(user) => user,
                Err(err) => {
                    error!("{:?}", err);
                    if let sqlx::error::Error::RowNotFound = err {
                        return Err(Status::not_found("User not found"));
                    }
                    return Err(Status::internal("Something went wrong"));
                }
            };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(UnfollowUserResponse {
                message: format!("User {} unfollowed {}", user.0, user.1),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn followers(
        &self,
        request: Request<FollowersRequest>,
    ) -> Result<Response<FollowersResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get User Followers Request {:?}", input);

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

        let users = match UserRepositoryImpl::followers(
            &mut transaction,
            &input.id,
            input.limit,
            id,
            created_at,
            input.by_user.as_deref(),
        )
        .await
        {
            Ok(users) => users,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let next_cursor = users
            .iter()
            .nth(input.limit as usize - 1)
            .map(|item| format!("{}_{}", item.id, item.created_at.to_rfc3339()));

        let users = users.iter().map(|user| FullUser::from(user)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(FollowersResponse { users, next_cursor })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn following(
        &self,
        request: Request<FollowingRequest>,
    ) -> Result<Response<FollowingResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get User Follows Request {:?}", input);

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

        let users = match UserRepositoryImpl::following(
            &mut transaction,
            &input.id,
            input.limit,
            id,
            created_at,
            input.by_user.as_deref(),
        )
        .await
        {
            Ok(users) => users,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let next_cursor = users
            .iter()
            .nth(input.limit as usize - 1)
            .map(|item| format!("{}_{}", item.id, item.created_at.to_rfc3339()));

        let users = users.iter().map(|user| FullUser::from(user)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(FollowingResponse { users, next_cursor })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}
