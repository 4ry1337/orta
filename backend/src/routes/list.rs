use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Extension, Json,
};
use axum_core::response::IntoResponse;
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use serde::Deserialize;
use serde_json::json;
use shared::{
    common,
    list::{
        list_service_client::ListServiceClient, AddArticleRequest, ArticlesRequest, CreateRequest,
        DeleteRequest, GetRequest, RemoveArticleRequest, SearchRequest, UpdateRequest,
    },
    models::{article_model::FullArticle, enums::Visibility, list_model::List},
    utils::jwt::{AccessToken, AccessTokenPayload, JWT},
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
}

pub async fn get_lists(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    Query(query): Query<ListsQueryParams>,
    Query(cursor): Query<CursorPagination>,
    State(_state): State<AppState>,
) -> Response {
    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();
    info!("Get Lists Request {:?} {:?}", query, cursor);

    match ListServiceClient::new(channel)
        .search(SearchRequest {
            query: query.query,
            cursor: cursor.cursor,
            limit: cursor.limit,
            by_user,
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
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Path(path): Path<PathParams>,
) -> Response {
    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();

    let list_id = match path.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!("Get List Request {:?}", list_id);

    match ListServiceClient::new(channel)
        .get(GetRequest { list_id, by_user })
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

pub async fn get_list_articles(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    Query(cursor): Query<CursorPagination>,
    Path(params): Path<PathParams>,
) -> Response {
    info!("Get User Articles Request {:?}", params);

    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();

    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    match ListServiceClient::new(channel)
        .articles(ArticlesRequest {
            cursor: cursor.cursor,
            limit: cursor.limit,
            list_id,
            by_user,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<FullArticle> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res
                        .articles
                        .iter()
                        .map(|article| FullArticle::from(article))
                        .collect()
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
        .create(CreateRequest {
            user_id: user.user_id,
            label: payload.label,
            image: payload.image,
            visibility: common::Visibility::from(payload.visibility) as i32,
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
        .delete(DeleteRequest {
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
        .update(UpdateRequest {
            label: payload.label,
            image: payload.image,
            visibility: payload
                .visibility
                .map(|visibility| common::Visibility::from(visibility) as i32),
            user_id: user.user_id,
            list_id,
        })
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
        .add_article(AddArticleRequest {
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
        .remove_article(RemoveArticleRequest {
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
