use std::sync::Arc;

use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use shared::{
    models::user_model::{self, UpdateUser},
    repositories::prelude::*,
    resource_proto::{
        user_service_server::UserService, FollowUserRequest, IntParam, QueryParams, StringParam,
        UpdateUserRequest, User, UserVec,
    },
    utils::params::{self, Filter},
};
use tonic::{Request, Response, Status};
use tracing::error;

use crate::application::AppState;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub state: Arc<AppState>,
}

pub struct W<T>(pub T);

/// Converts chrono's `DateTime<Utc>` to `Timestamp`
impl From<W<DateTime<Utc>>> for Timestamp {
    fn from(dt: W<DateTime<Utc>>) -> Self {
        let dt = dt.0;
        prost_types::Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        }
    }
}

impl From<W<Option<DateTime<Utc>>>> for Option<Timestamp> {
    fn from(dt: W<Option<DateTime<Utc>>>) -> Self {
        let dt = dt.0;
        match dt {
            Some(dt) => Some(Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }),
            None => None,
        }
    }
}

/// Converts proto timestamp to chrono's DateTime<Utc>
impl From<W<Timestamp>> for DateTime<Utc> {
    fn from(val: W<Timestamp>) -> Self {
        let mut value = val.0;
        // A call to `normalize` should capture all out-of-bound sitations hopefully
        // ensuring a panic never happens! Ideally this implementation should be
        // deprecated in favour of TryFrom but unfortunately having `TryFrom` along with
        // `From` causes a conflict.
        value.normalize();
        DateTime::from_timestamp(value.seconds, value.nanos as u32)
            .expect("invalid or out-of-range datetime")
    }
}

impl From<W<Option<Timestamp>>> for Option<DateTime<Utc>> {
    fn from(val: W<Option<Timestamp>>) -> Self {
        // A call to `normalize` should capture all out-of-bound sitations hopefully
        // ensuring a panic never happens! Ideally this implementation should be
        // deprecated in favour of TryFrom but unfortunately having `TryFrom` along with
        // `From` causes a conflict.
        match val.0 {
            Some(mut value) => {
                value.normalize();
                Some(
                    DateTime::from_timestamp(value.seconds, value.nanos as u32)
                        .expect("invalid or out-of-range datetime"),
                )
            }
            None => None,
        }
    }
}

pub fn into_user_proto(user_model: &user_model::User) -> User {
    User {
        id: user_model.id,
        email: user_model.email.clone(),
        email_verified: W(user_model.email_verified).into(),
        username: user_model.username.clone(),
        image: user_model.image.clone(),
        role: user_model.role as i32,
        following_count: user_model.following_count,
        follower_count: user_model.follower_count,
        approved_at: W(user_model.approved_at).into(),
        deleted_at: W(user_model.deleted_at).into(),
    }
}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn get_users(&self, request: Request<QueryParams>) -> Result<Response<UserVec>, Status> {
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

        let filter = Filter::from(&input);

        let users = match UserRepositoryImpl::find_all(&mut transaction, &filter).await {
            Ok(users) => users,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let users = users.iter().map(|user| into_user_proto(user)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(UserVec { total, users })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn get_user(&self, request: Request<StringParam>) -> Result<Response<User>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let user = match UserRepositoryImpl::find_by_username(&mut transaction, &input.value).await
        {
            Ok(user) => user,
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("User not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let user = into_user_proto(&user);

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(user)),
            Err(err) => {
                error!("{:#?}", err);
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
                username: input.username.to_owned(),
                image: None,
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

        let user = into_user_proto(&user);

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(user)),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn delete_user(&self, request: Request<IntParam>) -> Result<Response<User>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let user = match UserRepositoryImpl::find(&mut transaction, input.value).await {
            Ok(user) => user,
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("User not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let user = match UserRepositoryImpl::delete(&mut transaction, user.id).await {
            Ok(user) => user,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let user = into_user_proto(&user);

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(user)),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
    async fn verify_email(&self, request: Request<StringParam>) -> Result<Response<User>, Status> {
        unimplemented!()
    }
    async fn follow_user(
        &self,
        request: Request<FollowUserRequest>,
    ) -> Result<Response<User>, Status> {
        unimplemented!()
    }
}
