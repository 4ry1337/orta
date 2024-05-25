use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use minio::s3::args::{GetObjectArgs, PutObjectApiArgs};
use shared::configuration::CONFIG;
use tracing::{debug, error, info};

use crate::{application::AppState, utils::params::PathParams};

pub async fn get_asset(State(state): State<AppState>, Path(params): Path<PathParams>) -> Response {
    debug!(?params);

    let asset_name = match params.asset_name {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!("Get Asset Request {}", asset_name);

    let res = match state
        .storage
        .get_object_old(&GetObjectArgs::new(&CONFIG.storage.bucket_name, &asset_name).unwrap())
        .await
    {
        Ok(put_object_response) => put_object_response,
        Err(err) => {
            error!(?err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    info!("asset `{asset_name}` retrived.");

    (StatusCode::OK, res.bytes().await.unwrap()).into_response()
}

#[derive(Debug, TryFromMultipart)]
pub struct UploadAssetRequest {
    #[form_data(limit = "unlimited")]
    asset: FieldData<Bytes>,
}

pub async fn post_asset(
    State(state): State<AppState>,
    TypedMultipart(UploadAssetRequest { asset }): TypedMultipart<UploadAssetRequest>,
) -> Response {
    info!("Post Asset Request");

    let file_name = match asset.metadata.file_name {
        Some(file_name) => file_name,
        None => return (StatusCode::NOT_ACCEPTABLE, "No Filename").into_response(),
    };

    let data = asset.contents;

    let asset_name = format!(
        "{}_{}",
        chrono::Utc::now().format("%d-%m-%Y_%H:%M:%S"),
        &file_name
    );

    match state
        .storage
        .put_object_api(
            &mut PutObjectApiArgs::new(&CONFIG.storage.bucket_name, &asset_name, &data).unwrap(),
        )
        .await
    {
        Ok(put_object_response) => put_object_response,
        Err(err) => {
            error!(?err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    info!("asset `{asset_name}` uploaded.");

    (StatusCode::OK, asset_name).into_response()
}
