use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Extension, Json,
};
use axum_core::response::IntoResponse;
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use serde::Deserialize;
use serde_aux::prelude::default_i64;
use serde_json::json;
use shared::{
    models::{article_model::FullArticle, series_model::Series},
    series::{
        series_service_client::SeriesServiceClient, AddArticleRequest, ArticlesRequest,
        CreateRequest, DeleteRequest, GetRequest, RemoveArticleRequest, ReorderArticleRequest,
        SearchRequest, UpdateRequest,
    },
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
pub struct SeriesQueryParams {
    query: Option<String>,
}

pub async fn get_serieses(
    Extension(channel): Extension<Channel>,
    Query(query): Query<SeriesQueryParams>,
    Query(cursor): Query<CursorPagination>,
    State(_state): State<AppState>,
) -> Response {
    info!("Get Series Request {:?} {:?}", query, cursor);

    match SeriesServiceClient::new(channel)
        .search(SearchRequest {
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
                Json(json!(ResultPaging::<Series> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res
                        .series
                        .iter()
                        .map(|series| Series::from(series))
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

pub async fn get_series(
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Path(path): Path<PathParams>,
) -> Response {
    let series_id = match path.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!("Get Series Request {:?}", series_id);

    match SeriesServiceClient::new(channel)
        .get(GetRequest { series_id })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(Series::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SeriesArticlePagination {
    pub cursor: Option<f32>,
    #[serde(default = "default_i64::<25>")]
    pub limit: i64,
}

pub async fn get_series_articles(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    Query(cursor): Query<SeriesArticlePagination>,
    Path(params): Path<PathParams>,
) -> Response {
    info!("Get Series Articles Request {:?} {:?}", params, cursor);

    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();

    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    match SeriesServiceClient::new(channel)
        .articles(ArticlesRequest {
            order: cursor.cursor,
            limit: cursor.limit,
            series_id,
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
pub struct PostSeriesRequestBody {
    pub label: String,
    pub image: Option<String>,
}

pub async fn post_series(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Json(payload): Json<PostSeriesRequestBody>,
) -> Response {
    info!("Post Series Request {:?} {:?}", user, payload);
    match SeriesServiceClient::new(channel)
        .create(CreateRequest {
            user_id: user.user_id,
            image: payload.image,
            label: payload.label,
        })
        .await
    {
        Ok(res) => (
            StatusCode::CREATED,
            Json(json!(Series::from(res.get_ref()))),
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
pub struct PatchSeriesRequestBody {
    pub label: Option<String>,
    pub image: Option<String>,
}

pub async fn patch_series(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchSeriesRequestBody>,
) -> Response {
    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Patch Series Request {:?} {:?} {:?}",
        series_id, user, payload
    );

    match SeriesServiceClient::new(channel)
        .update(UpdateRequest {
            user_id: user.user_id,
            series_id,
            label: payload.label,
            image: payload.image,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(Series::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

pub async fn delete_series(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!("Patch Series Request {:?} {:?}", series_id, user);

    match SeriesServiceClient::new(channel)
        .delete(DeleteRequest {
            user_id: user.user_id,
            series_id,
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
pub struct AddSeriesArticleRequestBody {
    pub article_id: String,
}

pub async fn put_series_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<AddSeriesArticleRequestBody>,
) -> Response {
    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Put Article to Series Request {:?} {:?} {:?}",
        payload, series_id, user
    );

    match SeriesServiceClient::new(channel)
        .add_article(AddArticleRequest {
            user_id: user.user_id,
            series_id,
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
pub struct PatchSeriesArticleRequestBody {
    pub article_id: String,
    pub order: f32,
}

pub async fn patch_series_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchSeriesArticleRequestBody>,
) -> Response {
    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Patch Series's Article Request {:?} {:?} {:?}",
        payload, series_id, user
    );

    match SeriesServiceClient::new(channel)
        .reorder_article(ReorderArticleRequest {
            user_id: user.user_id,
            series_id,
            article_id: payload.article_id,
            order: payload.order,
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
pub struct DeleteSeriesArticleRequestBody {
    pub article_id: String,
}

pub async fn delete_series_article(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<DeleteSeriesArticleRequestBody>,
) -> Response {
    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Delete Article from Series Request {:?} {:?} {:?}",
        payload, series_id, user
    );

    match SeriesServiceClient::new(channel)
        .remove_article(RemoveArticleRequest {
            user_id: user.user_id,
            series_id,
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
