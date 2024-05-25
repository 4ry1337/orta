use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    Extension, Json,
};
use axum_core::response::IntoResponse;
use axum_extra::extract::Query;
use serde::Deserialize;
use serde_json::json;
use shared::{
    models::article_model::{Article, FullArticle},
    resource_proto::{
        article_service_client::ArticleServiceClient, AddAuthorRequest, CreateArticleRequest,
        DeleteArticleRequest, GetArticleRequest, GetArticlesRequest, QueryParams,
        RemoveAuthorRequest, UpdateArticleRequest,
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

#[derive(Debug, Deserialize)]
pub struct ArticlesQueryParams {
    usernames: Option<Vec<String>>,
    list_id: Option<String>,
    series_id: Option<String>,
}

pub async fn get_articles(
    Extension(channel): Extension<Channel>,
    Query(query): Query<ArticlesQueryParams>,
    Query(pagination): Query<Pagination>,
    State(_state): State<AppState>,
) -> Response {
    info!("Get Articles Request {:#?} {:#?}", query, pagination);
    match ArticleServiceClient::new(channel)
        .get_articles(GetArticlesRequest {
            usernames: query.usernames.unwrap_or_default(),
            list_id: query.list_id,
            series_id: query.series_id,
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
                Json(json!(ResultPaging::<FullArticle> {
                    total: res.total,
                    pagination: Metadata::new(res.total, pagination.per_page, pagination.page),
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
            (status_code, message).into_response()
        }
    }
}

pub async fn get_article(
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.asset_name {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!("Get Article Request {}", article_id);
    match ArticleServiceClient::new(channel)
        .get_article(GetArticleRequest { article_id })
        .await
    {
        Ok(res) => (
            StatusCode::OK,
            Json(json!(FullArticle::from(res.get_ref()))),
        )
            .into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostArticleRequestBody {
    title: String,
}

pub async fn post_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Json(payload): Json<PostArticleRequestBody>,
) -> Response {
    info!("Post Articles Request {:#?} {:#?}", user, payload);

    match ArticleServiceClient::new(channel)
        .create_article(CreateArticleRequest {
            title: payload.title,
            user_id: user.user_id,
        })
        .await
    {
        Ok(res) => (
            StatusCode::CREATED,
            Json(json!(Article::from(res.get_ref()))),
        )
            .into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchArticleRequestBody {
    pub title: Option<String>,
}

pub async fn patch_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchArticleRequestBody>,
) -> Response {
    let article_id = match params.asset_name {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!(
        "Patch Articles Request {:#?} {:#?} {:#?}",
        user, article_id, payload
    );
    match ArticleServiceClient::new(channel)
        .update_article(UpdateArticleRequest {
            title: payload.title,
            user_id: user.user_id,
            article_id,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(Article::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

pub async fn delete_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.asset_name {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!("Delete Articles Request {:#?} {:#?}", user, article_id);
    match ArticleServiceClient::new(channel)
        .delete_article(DeleteArticleRequest {
            user_id: user.user_id,
            article_id,
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
pub struct PutAuthorRequestBody {
    pub user_id: String,
}

pub async fn put_author(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PutAuthorRequestBody>,
) -> Response {
    let article_id = match params.asset_name {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Put Author to Articles Request {:#?} {:#?} {:#?}",
        user, payload, article_id
    );

    match ArticleServiceClient::new(channel)
        .add_author(AddAuthorRequest {
            author_id: user.user_id,
            article_id,
            user_id: payload.user_id,
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
pub struct DeleteAuthorRequestBody {
    pub user_id: String,
}
pub async fn delete_author(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<DeleteAuthorRequestBody>,
) -> Response {
    let article_id = match params.asset_name {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Delete Author to Articles Request {:#?} {:#?} {:#?}",
        user, payload, article_id
    );

    match ArticleServiceClient::new(channel)
        .remove_author(RemoveAuthorRequest {
            author_id: user.user_id,
            article_id,
            user_id: payload.user_id,
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
