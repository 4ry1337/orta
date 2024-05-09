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
    models::series_model::Series,
    resource_proto::{
        series_service_client::SeriesServiceClient, AddArticleSeriesRequest, CreateSeriesRequest,
        DeleteSeriesRequest, GetSeriesRequest, GetSeriesesRequest, QueryParams,
        RemoveArticleSeriesRequest, UpdateSeriesRequest,
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
pub struct SeriesQueryParams {
    label: Option<String>,
    user_id: Option<i32>,
}

pub async fn get_serieses(
    Query(query): Query<SeriesQueryParams>,
    Query(pagination): Query<Pagination>,
    State(state): State<AppState>,
) -> Response {
    match SeriesServiceClient::new(state.resource_server.clone())
        .get_serieses(GetSeriesesRequest {
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
                Json(json!(ResultPaging::<Series> {
                    total: res.total,
                    pagination: Metadata::new(res.total, pagination.per_page, pagination.page),
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
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

pub async fn get_series(State(state): State<AppState>, Path(path): Path<PathParams>) -> Response {
    let series_slug = match path.series_slug {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    match SeriesServiceClient::new(state.resource_server.clone())
        .get_series(GetSeriesRequest { series_slug })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(Series::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostSeriesRequestBody {
    pub label: String,
    pub image: Option<String>,
}

pub async fn post_series(
    Extension(user): Extension<AccessTokenPayload>,
    State(state): State<AppState>,
    Json(payload): Json<PostSeriesRequestBody>,
) -> Response {
    match SeriesServiceClient::new(state.resource_server.clone())
        .create_series(CreateSeriesRequest {
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
            error!("{:#?}", err);
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
    Extension(user): Extension<AccessTokenPayload>,
    State(state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchSeriesRequestBody>,
) -> Response {
    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    match SeriesServiceClient::new(state.resource_server.clone())
        .update_series(UpdateSeriesRequest {
            user_id: user.user_id,
            series_id,
            label: payload.label,
            image: payload.image,
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

pub async fn delete_series(
    Extension(user): Extension<AccessTokenPayload>,
    State(state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    match SeriesServiceClient::new(state.resource_server.clone())
        .delete_series(DeleteSeriesRequest {
            user_id: user.user_id,
            series_id,
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
pub struct AddSeriesArticleRequestBody {
    pub article_id: i32,
}

pub async fn put_series_article(
    Extension(user): Extension<AccessTokenPayload>,
    State(state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<AddSeriesArticleRequestBody>,
) -> Response {
    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    match SeriesServiceClient::new(state.resource_server.clone())
        .add_article(AddArticleSeriesRequest {
            user_id: user.user_id,
            series_id,
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
pub struct DeleteSeriesArticleRequestBody {
    pub article_id: i32,
}

pub async fn delete_series_article(
    Extension(user): Extension<AccessTokenPayload>,
    State(state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<DeleteSeriesArticleRequestBody>,
) -> Response {
    let series_id = match params.series_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    match SeriesServiceClient::new(state.resource_server.clone())
        .remove_article(RemoveArticleSeriesRequest {
            user_id: user.user_id,
            series_id,
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
