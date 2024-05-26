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
        DeleteSeriesRequest, GetSeriesRequest, GetSeriesesRequest, RemoveArticleSeriesRequest,
        UpdateSeriesRequest,
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
pub struct SeriesQueryParams {
    label: Option<String>,
    user_id: Option<String>,
}

pub async fn get_serieses(
    Extension(channel): Extension<Channel>,
    Query(query): Query<SeriesQueryParams>,
    Query(cursor): Query<CursorPagination>,
    State(_state): State<AppState>,
) -> Response {
    info!("Get Series Request {:?} {:?}", query, cursor);

    match SeriesServiceClient::new(channel)
        .get_serieses(GetSeriesesRequest {
            user_id: query.user_id,
            query: query.label,
            limit: cursor.limit,
            cursor: cursor.cursor,
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
        .get_series(GetSeriesRequest { series_id })
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
        .update_series(UpdateSeriesRequest {
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
        .delete_series(DeleteSeriesRequest {
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
        .add_article(AddArticleSeriesRequest {
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
        .remove_article(RemoveArticleSeriesRequest {
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
