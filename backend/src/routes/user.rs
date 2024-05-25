use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::Deserialize;
use serde_json::json;
use shared::{
    models::user_model::User,
    resource_proto::{
        user_service_client::UserServiceClient, GetUserRequest, GetUsersRequest, QueryParams,
        UpdateUserRequest,
    },
    utils::jwt::AccessTokenPayload,
};
use tonic::transport::Channel;
use tracing::{error, info};

use crate::{
    application::AppState,
    utils::{
        mapper::code_to_statudecode,
        params::{Metadata, Pagination, PathParams, ResultPaging},
    },
};

pub async fn get_users(
    Extension(channel): Extension<Channel>,
    Query(query): Query<Pagination>,
    State(_state): State<AppState>,
) -> Response {
    match UserServiceClient::new(channel)
        .get_users(GetUsersRequest {
            query: None,
            params: Some(QueryParams {
                order_by: None,
                per_page: Some(query.per_page),
                page: Some(query.page),
            }),
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<User> {
                    total: res.total,
                    pagination: Metadata::new(res.total, query.per_page, query.page),
                    items: res.users.iter().map(|user| User::from(user)).collect()
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
        .get_user(GetUserRequest { username })
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
