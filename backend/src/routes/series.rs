use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    Json,
};
use axum_core::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    application::AppState,
    models::series_model::{CreateSeries, UpdateSeries},
    repositories::series_repository::{SeriesRepository, SeriesRepositoryImpl},
};

pub async fn get_series(State(state): State<Arc<AppState>>) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let series = match SeriesRepositoryImpl::find_all(&mut transaction).await {
        Ok(series) => series,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(series))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn get_series_by_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let series = match SeriesRepositoryImpl::find_by_user(&mut transaction, user_id).await {
        Ok(series) => series,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(series))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostSeriesRequestBody {
    pub user_id: i32,
    pub label: String,
    pub image: Option<String>,
}

pub async fn post_series(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PostSeriesRequestBody>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let create_series = CreateSeries {
        user_id: payload.user_id,
        label: payload.label,
        image: payload.image,
    };
    let series = match SeriesRepositoryImpl::create(&mut transaction, &create_series).await {
        Ok(series) => series,
        Err(err) => {
            //TODO: series not found
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(series))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchSeriesRequestBody {
    pub label: Option<String>,
    pub image: Option<String>,
}

pub async fn patch_series(
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
    Json(payload): Json<PatchSeriesRequestBody>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let update_series = UpdateSeries {
        id: series_id,
        label: payload.label,
        image: payload.image,
    };
    let series = match SeriesRepositoryImpl::update(&mut transaction, &update_series).await {
        Ok(series) => series,
        Err(err) => {
            //TODO: series not found
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(series))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn delete_series(
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let series = match SeriesRepositoryImpl::delete(&mut transaction, series_id).await {
        Ok(series) => series,
        Err(err) => {
            //TODO: series not found
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(series))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn get_series_articles(
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let series = match SeriesRepositoryImpl::find_articles(&mut transaction, series_id).await {
        Ok(series) => series,
        Err(err) => {
            //TODO: series not found
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(series))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn put_series_article(
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
    Path(article_id): Path<i32>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let series =
        match SeriesRepositoryImpl::add_article(&mut transaction, series_id, article_id).await {
            Ok(series) => series,
            Err(err) => {
                //TODO: series not found
                error!("{:#?}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
            }
        };
    match transaction.commit().await {
        Ok(_) => (
            StatusCode::OK,
            format!("Article {} added to Series {}", series.0, series.1),
        )
            .into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn delete_series_article(
    State(state): State<Arc<AppState>>,
    Path(series_id): Path<i32>,
    Path(article_id): Path<i32>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let series =
        match SeriesRepositoryImpl::remove_article(&mut transaction, series_id, article_id).await {
            Ok(series) => series,
            Err(err) => {
                //TODO: series not found
                //TODO: article not found
                error!("{:#?}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
            }
        };
    match transaction.commit().await {
        Ok(_) => (
            StatusCode::OK,
            format!("Article {} deleted from Series {}", series.0, series.1),
        )
            .into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}
