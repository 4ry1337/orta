use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::Deserialize;
use serde_json::json;
use shared::{
    models::user_model::{FullUser, User},
    resource_proto::{
        user_service_client::UserServiceClient, FollowUserRequest, FollowersRequest,
        FollowingRequest, GetUserRequest, GetUsersRequest, UnfollowUserRequest, UpdateUserRequest,
    },
    utils::jwt::AccessTokenPayload,
};
use tonic::transport::Channel;
use tracing::{error, info};

use crate::{
    application::AppState,
    utils::{
        mapper::code_to_statudecode,
        params::{CursorPagination, PathParams, ResultPaging},
    },
};

#[derive(Debug, Deserialize)]
pub struct UsersQueryParams {
    query: Option<String>,
}

pub async fn get_users(
    user: Option<Extension<AccessTokenPayload>>,
    Extension(channel): Extension<Channel>,
    Query(query): Query<UsersQueryParams>,
    Query(cursor): Query<CursorPagination>,
    State(_state): State<AppState>,
) -> Response {
    match UserServiceClient::new(channel)
        .get_users(GetUsersRequest {
            query: query.query,
            limit: cursor.limit,
            cursor: cursor.cursor,
            by_user: user.map(|u| u.user_id.clone()),
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<FullUser> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res.users.iter().map(|user| FullUser::from(user)).collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn get_user(
    user: Option<Extension<AccessTokenPayload>>,
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    info!("Get User Request");
    let username = match params.username {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    match UserServiceClient::new(channel)
        .get_user(GetUserRequest {
            username,
            by_user: user.map(|u| u.user_id.clone()),
        })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(FullUser::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

// #[derive(Debug, Deserialize)]
// pub struct CreateUserRequestBody {
//     pub username: String,
//     pub email: String,
//     pub email_verified: Option<DateTime<Utc>>,
//     pub image: Option<String>,
// }
//
// pub async fn post_user(
// State(_state): State<AppState>,
//     Json(payload): Json<CreateUserRequestBody>,
// ) -> Response {
//     match UserServiceClient::new(state.auth_server.clone())
//         .(GetUserRequest { username })
//         .await
//     {
//         Ok(res) => (StatusCode::OK, Json(json!(User::from(res.get_ref())))).into_response(),
//         Err(err) => {
//             error!("{:#?}", err);
//             let message = err.message().to_string();
//             let status_code = code_to_statudecode(err.code());
//             return (status_code, message).into_response();
//         }
//     }
// }

#[derive(Debug, Deserialize)]
pub struct PatchUserRequestBody {
    pub username: Option<String>,
    pub image: Option<Vec<u8>>,
    pub bio: Option<String>,
    pub url: Vec<String>,
}

pub async fn patch_user(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Json(payload): Json<PatchUserRequestBody>,
) -> Response {
    match UserServiceClient::new(channel)
        .update_user(UpdateUserRequest {
            id: user.user_id,
            username: payload.username,
            image: payload.image,
            bio: payload.bio,
            url: payload.url,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(User::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn follow(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let user_id = match params.user_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    match UserServiceClient::new(channel)
        .follow_user(FollowUserRequest {
            user_id: user.user_id,
            target_id: user_id,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn unfollow(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let user_id = match params.user_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    match UserServiceClient::new(channel)
        .unfollow_user(UnfollowUserRequest {
            user_id: user.user_id,
            target_id: user_id,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn get_followers(
    Extension(channel): Extension<Channel>,
    Query(cursor): Query<CursorPagination>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let user_id = match params.user_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    match UserServiceClient::new(channel)
        .followers(FollowersRequest {
            id: user_id,
            limit: cursor.limit,
            cursor: cursor.cursor,
            by_user: None,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<FullUser> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res.users.iter().map(|user| FullUser::from(user)).collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn get_following(
    Extension(channel): Extension<Channel>,
    Query(cursor): Query<CursorPagination>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let user_id = match params.user_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    match UserServiceClient::new(channel)
        .following(FollowingRequest {
            id: user_id,
            limit: cursor.limit,
            cursor: cursor.cursor,
            by_user: None,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<FullUser> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res.users.iter().map(|user| FullUser::from(user)).collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

// pub async fn delete_user(
// State(_state): State<AppState>,
//     Path(params): Path<PathParams>,
// ) -> Response {
//     let user_id = match params.user_id {
//         Some(v) => v,
//         None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
//     };
//     unimplemented!()
// }
