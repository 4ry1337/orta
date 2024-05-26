use axum::{
    body::Bytes,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use shared::storage_proto::{
    storage_service_client::StorageServiceClient, RetriveRequest, StoreRequest,
};
use tonic::{codec::CompressionEncoding, transport::Channel};
use tracing::{debug, error, info};

use crate::utils::{mapper::code_to_statudecode, params::PathParams};

pub async fn get_asset(
    Extension(channel): Extension<Channel>,
    Path(params): Path<PathParams>,
) -> Response {
    debug!(?params);

    let asset_name = match params.asset_name {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!("Get Asset Request {}", asset_name);

    match StorageServiceClient::new(channel)
        .accept_compressed(CompressionEncoding::Gzip)
        .max_decoding_message_size(50 * 1024 * 1024)
        .retrive(RetriveRequest { asset_name })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().data.to_owned()).into_response(),
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

#[derive(Debug, TryFromMultipart)]
pub struct UploadAssetRequest {
    #[form_data(limit = "unlimited")]
    asset: FieldData<Bytes>,
}

pub async fn post_asset(
    Extension(channel): Extension<Channel>,
    TypedMultipart(UploadAssetRequest { asset }): TypedMultipart<UploadAssetRequest>,
) -> Response {
    info!("Post Asset Request {:?}", asset.metadata);

    let file_name = match asset.metadata.file_name {
        Some(file_name) => file_name,
        None => return (StatusCode::NOT_ACCEPTABLE, "No Filename").into_response(),
    };

    match StorageServiceClient::new(channel)
        .accept_compressed(CompressionEncoding::Gzip)
        .max_encoding_message_size(50 * 1024 * 1024)
        .max_decoding_message_size(50 * 1024 * 1024)
        .store(StoreRequest {
            asset_data: asset.contents.to_vec(),
            asset_name: file_name,
            content_type: asset.metadata.content_type,
        })
        .await
    {
        Ok(res) => (StatusCode::CREATED, res.get_ref().asset_name.to_owned()).into_response(),
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}
