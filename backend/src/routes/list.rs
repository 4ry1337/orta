use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Response,
    Extension, Json,
};
use axum_core::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;
use shared::{
    models::{enums::Visibility, list_model::List},
    resource_proto::{
        self, list_service_client::ListServiceClient, AddArticleListRequest, CreateListRequest,
        DeleteListRequest, GetListRequest, GetListsRequest,
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
pub struct ListsQueryParams {
    query: Option<String>,
    user_id: Option<String>,
}

pub async fn get_lists(
    Extension(channel): Extension<Channel>,
    Query(query): Query<ListsQueryParams>,
    Query(cursor): Query<CursorPagination>,
    State(_state): State<AppState>,
) -> Response {
    info!("Get Lists Request {:?} {:?}", query, cursor);

    match ListServiceClient::new(channel)
        .get_lists(GetListsRequest {
            user_id: query.user_id,
            query: query.query,
            cursor: cursor.cursor,
            limit: cursor.limit,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<List> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res.lists.iter().map(|list| List::from(list)).collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

pub async fn get_list(
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Path(path): Path<PathParams>,
) -> Response {
    let list_id = match path.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!("Get List Request {:?}", list_id);

    match ListServiceClient::new(channel)
        .get_list(GetListRequest { list_id })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(List::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostListRequestBody {
    pub label: String,
    pub image: Option<String>,
    pub visibility: Visibility,
}

pub async fn post_list(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Json(payload): Json<PostListRequestBody>,
) -> Response {
    info!("Post List Request {:?} {:?}", user, payload);

    match ListServiceClient::new(channel)
        .create_list(CreateListRequest {
            user_id: user.user_id,
            label: payload.label,
            image: payload.image,
            visibility: resource_proto::Visibility::from(payload.visibility) as i32,
        })
        .await
    {
        Ok(res) => (StatusCode::CREATED, Json(json!(List::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

pub async fn delete_list(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(path): Path<PathParams>,
) -> Response {
    let list_id = match path.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!("Delete List Request {:?} {:?}", user, list_id);

    match ListServiceClient::new(channel)
        .delete_list(DeleteListRequest {
            user_id: user.user_id,
            list_id,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct PatchListRequestBody {
    pub label: Option<String>,
    pub image: Option<String>,
    pub visibility: Option<Visibility>,
}

pub async fn patch_list(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchListRequestBody>,
) -> Response {
    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!("Patch List Request {:?} {:?} {:?}", user, list_id, payload);

    match ListServiceClient::new(channel)
        .delete_list(DeleteListRequest {
            user_id: user.user_id,
            list_id,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PutListArticleRequestBody {
    pub article_id: String,
}

pub async fn put_list_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PutListArticleRequestBody>,
) -> Response {
    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Put Article to List Request {:?} {:?} {:?}",
        payload, list_id, user
    );

    match ListServiceClient::new(channel)
        .add_article(AddArticleListRequest {
            user_id: user.user_id,
            list_id,
            article_id: payload.article_id,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteListArticleRequestBody {
    pub article_id: String,
}

pub async fn delete_list_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<DeleteListArticleRequestBody>,
) -> Response {
    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Delete Article to List Request {:?} {:?} {:?}",
        payload, list_id, user
    );

    match ListServiceClient::new(channel)
        .add_article(AddArticleListRequest {
            user_id: user.user_id,
            list_id,
            article_id: payload.article_id,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}
