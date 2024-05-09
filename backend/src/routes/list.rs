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
        DeleteListRequest, GetListRequest, GetListsRequest, QueryParams, UpdateListRequest,
    },
    utils::jwt::AccessTokenPayload,
};
use tracing::error;

use crate::{
    application::AppState,
    utils::{
        mapper::code_to_statudecode,
        params::{Metadata, Pagination, PathParams, ResultPaging},
    },
};

#[derive(Debug, Deserialize)]
pub struct ListsQueryParams {
    label: Option<String>,
    user_id: Option<i32>,
}

pub async fn get_lists(
    Query(query): Query<ListsQueryParams>,
    Query(pagination): Query<Pagination>,
    State(state): State<AppState>,
) -> Response {
    match ListServiceClient::new(state.resource_server.clone())
        .get_lists(GetListsRequest {
            user_id: query.user_id,
            query: query.label,
            params: Some(QueryParams {
                order_by: None,
                per_page: Some(pagination.per_page),
                page: Some(pagination.page),
            }),
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<List> {
                    total: res.total,
                    pagination: Metadata::new(res.total, pagination.per_page, pagination.page),
                    items: res.lists.iter().map(|list| List::from(list)).collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

pub async fn get_list(State(state): State<AppState>, Path(path): Path<PathParams>) -> Response {
    let list_slug = match path.list_slug {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    match ListServiceClient::new(state.resource_server.clone())
        .get_list(GetListRequest { list_slug })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(List::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostListRequestBody {
    pub user_id: i32,
    pub label: String,
    pub image: Option<String>,
    pub visibility: Visibility,
}

pub async fn post_list(
    Extension(user): Extension<AccessTokenPayload>,
    State(state): State<AppState>,
    Json(payload): Json<PostListRequestBody>,
) -> Response {
    match ListServiceClient::new(state.resource_server.clone())
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
            error!("{:#?}", err);
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
    Extension(user): Extension<AccessTokenPayload>,
    State(state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchListRequestBody>,
) -> Response {
    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    match ListServiceClient::new(state.resource_server.clone())
        .update_list(UpdateListRequest {
            user_id: user.user_id,
            list_id,
            label: payload.label,
            image: payload.image,
            visibility: payload
                .visibility
                .map(|visibility| resource_proto::Visibility::from(visibility) as i32),
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

pub async fn delete_list(
    Extension(user): Extension<AccessTokenPayload>,
    State(state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    match ListServiceClient::new(state.resource_server.clone())
        .delete_list(DeleteListRequest {
            user_id: user.user_id,
            list_id,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PutListArticleRequestBody {
    pub article_id: i32,
}

pub async fn put_list_article(
    Extension(user): Extension<AccessTokenPayload>,
    State(state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PutListArticleRequestBody>,
) -> Response {
    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    match ListServiceClient::new(state.resource_server.clone())
        .add_article(AddArticleListRequest {
            user_id: user.user_id,
            list_id,
            article_id: payload.article_id,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteListArticleRequestBody {
    pub article_id: i32,
}

pub async fn delete_list_article(
    Extension(user): Extension<AccessTokenPayload>,
    State(state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<DeleteListArticleRequestBody>,
) -> Response {
    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    match ListServiceClient::new(state.resource_server.clone())
        .add_article(AddArticleListRequest {
            user_id: user.user_id,
            list_id,
            article_id: payload.article_id,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}
