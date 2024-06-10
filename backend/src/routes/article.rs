use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Extension, Json,
};
use axum_core::response::IntoResponse;
use axum_extra::{
    extract::Query,
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
};
use serde::Deserialize;
use serde_json::json;
use shared::{
    article::{
        article_service_client::ArticleServiceClient, AddAuthorRequest, CreateRequest,
        DeleteRequest, EditRequest, GetRequest, HistoryRequest, LikeRequest, PublishRequest,
        RemoveAuthorRequest, SearchRequest, SetTagsRequest, UnlikeRequest, UpdateRequest,
    },
    models::article_model::{Article, ArticleVersion, FullArticle},
    utils::jwt::{AccessToken, AccessTokenPayload, JWT},
};
use tonic::{codec::CompressionEncoding, transport::Channel};
use tracing::{error, info};

use crate::{
    application::AppState,
    utils::{
        mapper::code_to_statudecode,
        params::{CursorPagination, PathParams, ResultPaging},
    },
};

#[derive(Debug, Deserialize)]
pub struct ArticlesQueryParams {
    query: Option<String>,
}

pub async fn search_articles(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    Query(query): Query<ArticlesQueryParams>,
    Query(cursor): Query<CursorPagination>,
    State(_state): State<AppState>,
) -> Response {
    info!("Get Articles Request {:?} {:?}", query, cursor);
    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();

    match ArticleServiceClient::new(channel)
        .accept_compressed(CompressionEncoding::Gzip)
        .max_decoding_message_size(50 * 1024 * 1024)
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
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

pub async fn get_article(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!("Get Article Request {}", article_id);
    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();

    match ArticleServiceClient::new(channel)
        .get(GetRequest {
            article_id,
            by_user,
        })
        .await
    {
        Ok(res) => (
            StatusCode::OK,
            Json(json!(FullArticle::from(res.get_ref()))),
        )
            .into_response(),
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostArticleRequestBody {
    title: String,
    description: Option<String>,
}

pub async fn post_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Json(payload): Json<PostArticleRequestBody>,
) -> Response {
    info!("Post Articles Request {:?} {:?}", user, payload);

    match ArticleServiceClient::new(channel)
        .create(CreateRequest {
            title: payload.title,
            description: payload.description,
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
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchArticleRequestBody {
    pub title: Option<String>,
    pub description: Option<String>,
}

pub async fn patch_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchArticleRequestBody>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!(
        "Patch Articles Request {:?} {:?} {:?}",
        user, article_id, payload
    );
    match ArticleServiceClient::new(channel)
        .update(UpdateRequest {
            title: payload.title,
            description: payload.description,
            user_id: user.user_id,
            article_id,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(Article::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:?}", err);
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
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!("Delete Articles Request {:?} {:?}", user, article_id);
    match ArticleServiceClient::new(channel)
        .delete(DeleteRequest {
            user_id: user.user_id,
            article_id,
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
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Put Author to Articles Request {:?} {:?} {:?}",
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
            error!("{:?}", err);
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
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Delete Author to Articles Request {:?} {:?} {:?}",
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
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EditArticleRequestBody {
    pub content: String,
}

pub async fn edit_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    Path(params): Path<PathParams>,
    Json(payload): Json<EditArticleRequestBody>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!(
        "Save Articles Version Request {:?} {:?} {:?}",
        user, article_id, payload
    );
    match ArticleServiceClient::new(channel)
        .edit(EditRequest {
            user_id: user.user_id,
            article_id,
            device_id: None,
            content: payload.content,
        })
        .await
    {
        Ok(res) => (
            StatusCode::OK,
            Json(json!(ArticleVersion::from(res.get_ref()))),
        )
            .into_response(),
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

pub async fn get_history(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    Path(params): Path<PathParams>,
    Query(cursor): Query<CursorPagination>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!(
        "Get Article History Request {:?} {:?} {:?}",
        user, article_id, cursor
    );
    match ArticleServiceClient::new(channel)
        .history(HistoryRequest {
            user_id: user.user_id,
            article_id,
            query: None,
            cursor: cursor.cursor,
            limit: cursor.limit,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<ArticleVersion> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res
                        .article_versions
                        .iter()
                        .map(|article_version| ArticleVersion::from(article_version))
                        .collect()
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

pub async fn like_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!("Like Articles Request {:?} {:?}", user.user_id, article_id);

    match ArticleServiceClient::new(channel)
        .like(LikeRequest {
            user_id: user.user_id,
            article_id,
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

pub async fn unlike_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Unlike Articles Request {:?} {:?}",
        user.user_id, article_id
    );

    match ArticleServiceClient::new(channel)
        .unlike(UnlikeRequest {
            user_id: user.user_id,
            article_id,
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

pub async fn publish(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Publish Articles Request {:?} {:?}",
        user.user_id, article_id
    );

    match ArticleServiceClient::new(channel)
        .publish(PublishRequest {
            user_id: user.user_id,
            article_id,
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
pub struct PutArticleTagsRequestBody {
    pub tags: Vec<String>,
}

pub async fn put_article_tags(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PutArticleTagsRequestBody>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Put Tags Articles Request {:?} {:?} {:?}",
        user.user_id, article_id, payload
    );

    match ArticleServiceClient::new(channel)
        .set_tags(SetTagsRequest {
            user_id: user.user_id,
            article_id,
            tags: payload.tags,
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
