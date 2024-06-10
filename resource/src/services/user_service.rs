use std::sync::Arc;

use chrono::{DateTime, Utc};
use shared::{
    common::{
        FullArticle, FullArticles, FullUser, FullUsers, List, Lists, MessageResponse, Series,
        Serieses, User,
    },
    models::user_model::UpdateUser,
    repositories::user_repository::{UserRepository, UserRepositoryImpl},
    user::{
        user_service_server::UserService, ArticlesRequest, DeleteRequest, FeedRequest,
        FollowRequest, FollowersRequest, FollowingRequest, GetRequest, ListsRequest, SearchRequest,
        SeriesesRequest, UnfollowRequest, UpdateRequest,
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
    async fn search(&self, request: Request<SearchRequest>) -> Result<Response<FullUsers>, Status> {
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
            Ok(_) => Ok(Response::new(FullUsers { users, next_cursor })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<FullUser>, Status> {
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

    async fn update(&self, request: Request<UpdateRequest>) -> Result<Response<User>, Status> {
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
                bio: input.bio.to_owned(),
                image: input.image.to_owned(),
                urls: input.urls.to_owned(),
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
            Ok(_) => Ok(Response::new(MessageResponse {
                message: format!("Deleted user: {}", user.id),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn follow(
        &self,
        request: Request<FollowRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Follow User Request {:?}", input);

        let user = match UserRepositoryImpl::follow(&mut transaction, &input.user_id, &input.target)
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
            Ok(_) => Ok(Response::new(MessageResponse {
                message: format!("User {} followed {}", user.0, user.1),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn unfollow(
        &self,
        request: Request<UnfollowRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
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
            match UserRepositoryImpl::unfollow(&mut transaction, &input.user_id, &input.target)
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
            Ok(_) => Ok(Response::new(MessageResponse {
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
    ) -> Result<Response<FullUsers>, Status> {
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
            &input.username,
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
            Ok(_) => Ok(Response::new(FullUsers { users, next_cursor })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn following(
        &self,
        request: Request<FollowingRequest>,
    ) -> Result<Response<FullUsers>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get User Following Request {:?}", input);

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
            &input.username,
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
            Ok(_) => Ok(Response::new(FullUsers { users, next_cursor })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn articles(
        &self,
        request: Request<ArticlesRequest>,
    ) -> Result<Response<FullArticles>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get User Articles Request {:?}", input);

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

        let articles = match UserRepositoryImpl::find_articles(
            &mut transaction,
            input.limit,
            id,
            created_at,
            input.by_user.as_deref(),
            Some(true),
            &input.username,
        )
        .await
        {
            Ok(articles) => articles,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let next_cursor = articles
            .iter()
            .nth(input.limit as usize - 1)
            .map(|item| format!("{}_{}", item.id, item.created_at.to_rfc3339()));

        let articles = articles
            .iter()
            .map(|article| FullArticle::from(article))
            .collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(FullArticles {
                articles,
                next_cursor,
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn drafts(
        &self,
        request: Request<ArticlesRequest>,
    ) -> Result<Response<FullArticles>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get Articles Request {:?}", input);

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

        let articles = match UserRepositoryImpl::find_articles(
            &mut transaction,
            input.limit,
            id,
            created_at,
            input.by_user.as_deref(),
            Some(false),
            &input.username,
        )
        .await
        {
            Ok(articles) => articles,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let next_cursor = articles
            .iter()
            .nth(input.limit as usize - 1)
            .map(|item| format!("{}_{}", item.id, item.created_at.to_rfc3339()));

        let articles = articles
            .iter()
            .map(|article| FullArticle::from(article))
            .collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(FullArticles {
                articles,
                next_cursor,
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn serieses(
        &self,
        request: Request<SeriesesRequest>,
    ) -> Result<Response<Serieses>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get User Serieses Request {:?}", input);

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

        let serieses = match UserRepositoryImpl::find_series(
            &mut transaction,
            input.limit,
            id,
            created_at,
            &input.username,
        )
        .await
        {
            Ok(serieses) => serieses,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let next_cursor = serieses
            .iter()
            .nth(input.limit as usize - 1)
            .map(|item| format!("{}_{}", item.id, item.created_at.to_rfc3339()));

        let serieses = serieses.iter().map(|series| Series::from(series)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(Serieses {
                series: serieses,
                next_cursor,
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn lists(&self, request: Request<ListsRequest>) -> Result<Response<Lists>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get Lists Request {:?}", input);

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

        let lists = match UserRepositoryImpl::find_lists(
            &mut transaction,
            input.limit,
            id,
            created_at,
            input.by_user.as_deref(),
            &input.username,
        )
        .await
        {
            Ok(lists) => lists,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let next_cursor = lists
            .iter()
            .nth(input.limit as usize - 1)
            .map(|item| format!("{}_{}", item.id, item.created_at.to_rfc3339()));

        let lists = lists.iter().map(|list| List::from(list)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(Lists { lists, next_cursor })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn feed(&self, request: Request<FeedRequest>) -> Result<Response<FullArticles>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get Feed Request {:?}", input);

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

        let articles = match UserRepositoryImpl::feed(
            &mut transaction,
            input.limit,
            id,
            created_at,
            &input.user_id,
        )
        .await
        {
            Ok(articles) => articles,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let next_cursor = articles
            .iter()
            .nth(input.limit as usize - 1)
            .map(|item| format!("{}_{}", item.id, item.created_at.to_rfc3339()));

        let articles = articles
            .iter()
            .map(|article| FullArticle::from(article))
            .collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(FullArticles {
                articles,
                next_cursor,
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}
