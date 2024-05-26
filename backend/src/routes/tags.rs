use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::Deserialize;
use serde_json::json;

use shared::{
    models::{enums::TagStatus, tag_model::Tag},
    resource_proto::{self, tag_service_client::TagServiceClient, GetTagsRequest},
};
use tonic::transport::Channel;
use tracing::{error, info};

use crate::{
    application::AppState,
    utils::{mapper::code_to_statudecode, params::CursorPagination},
};

#[derive(Debug, Deserialize)]
pub struct TagsQueryParams {
    user_id: Option<String>,
    article_id: Option<String>,
    tag_status: Option<TagStatus>,
}

pub async fn get_tags(
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Query(cursor): Query<CursorPagination>,
    Query(query): Query<TagsQueryParams>,
) -> Response {
    info!("Get Tags Request {:#?}", query);

    match TagServiceClient::new(channel)
        .get_tags(GetTagsRequest {
            user_id: query.user_id,
            article_id: query.article_id,
            tag_status: query
                .tag_status
                .map(|tag_status| resource_proto::TagStatus::from(tag_status) as i32),
            limit: cursor.limit,
            cursor: cursor.cursor,
        })
        .await
    {
        Ok(res) => {
            let tags: Vec<Tag> = res
                .get_ref()
                .tags
                .iter()
                .map(|tag| Tag::from(tag))
                .collect();
            (StatusCode::OK, Json(json!(tags))).into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

// pub async fn get_tag(
//     State(state): State<Arc<AppState>>,
//     Path(params): Path<PathParams>,
// ) -> Response {
//     let tag_id = match params.tag_id {
//         Some(v) => v,
//         None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
//     };
//
//     let mut transaction = match state.db.begin().await {
//         Ok(transaction) => transaction,
//         Err(err) => {
//             error!("{:#?}", err);
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     };
//
//     let tag = match TagRepositoryImpl::find(&mut transaction, tag_id).await {
//         Ok(tag) => tag,
//         Err(err) => {
//             error!("{:#?}", err);
//             if let sqlx::error::Error::RowNotFound = err {
//                 return (StatusCode::NOT_FOUND, "Tag not found").into_response();
//             }
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     };
//
//     match transaction.commit().await {
//         Ok(_) => (StatusCode::OK, Json(json!(tag))).into_response(),
//         Err(err) => {
//             error!("{:#?}", err);
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     }
// }

// #[derive(Debug, Deserialize)]
// pub struct PostTagRequestBody {
//     pub label: String,
//     pub tag_status: Option<String>,
// }
//
// pub async fn post_tag(
//     State(state): State<Arc<AppState>>,
//     Json(payload): Json<PostTagRequestBody>,
// ) -> Response {
//     let mut transaction = match state.db.begin().await {
//         Ok(transaction) => transaction,
//         Err(err) => {
//             error!("{:#?}", err);
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     };
//
//     let tag = match TagRepositoryImpl::create(
//         &mut transaction,
//         &CreateTag {
//             label: payload.label,
//             tag_status: payload
//                 .tag_status
//                 .as_ref()
//                 .and_then(|s| TagStatus::from_str(s).ok())
//                 .unwrap_or(TagStatus::Waiting),
//         },
//     )
//     .await
//     {
//         Ok(tag) => tag,
//         Err(err) => {
//             error!("{:#?}", err);
//             if let Some(database_error) = err.as_database_error() {
//                 if let Some(constraint) = database_error.constraint() {
//                     if constraint == "tags_label_key" {
//                         return (StatusCode::BAD_REQUEST, "Tag already exists").into_response();
//                     }
//                 }
//             }
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     };
//
//     match transaction.commit().await {
//         Ok(_) => (StatusCode::CREATED, Json(json!(tag))).into_response(),
//         Err(err) => {
//             error!("{:#?}", err);
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     }
// }
//
// #[derive(Debug, Deserialize)]
// pub struct PatchTagRequestBody {
//     pub label: Option<String>,
//     pub tag_status: Option<String>,
// }
//
// pub async fn patch_tag(
//     State(state): State<Arc<AppState>>,
//     Path(params): Path<PathParams>,
//     Json(payload): Json<PatchTagRequestBody>,
// ) -> Response {
//     let tag_id = match params.tag_id {
//         Some(v) => v,
//         None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
//     };
//
//     let mut transaction = match state.db.begin().await {
//         Ok(transaction) => transaction,
//         Err(err) => {
//             error!("{:#?}", err);
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     };
//
//     let tag = match TagRepositoryImpl::find(&mut transaction, tag_id).await {
//         Ok(tag) => tag,
//         Err(err) => {
//             error!("{:#?}", err);
//             if let sqlx::error::Error::RowNotFound = err {
//                 return (StatusCode::NOT_FOUND, "Tag not found").into_response();
//             }
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     };
//
//     let tag = match TagRepositoryImpl::update(
//         &mut transaction,
//         &UpdateTag {
//             id: tag.id,
//             label: payload.label,
//             tag_status: payload
//                 .tag_status
//                 .as_ref()
//                 .and_then(|s| TagStatus::from_str(s).ok()),
//         },
//     )
//     .await
//     {
//         Ok(tag) => tag,
//         Err(err) => {
//             error!("{:#?}", err);
//             if let Some(database_error) = err.as_database_error() {
//                 if let Some(constraint) = database_error.constraint() {
//                     if constraint == "tags_label_key" {
//                         return (StatusCode::BAD_REQUEST, "Tag already exists").into_response();
//                     }
//                 }
//             }
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     };
//
//     match transaction.commit().await {
//         Ok(_) => (StatusCode::OK, Json(json!(tag))).into_response(),
//         Err(err) => {
//             error!("{:#?}", err);
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     }
// }
//
// pub async fn delete_tag(
//     State(state): State<Arc<AppState>>,
//     Path(params): Path<PathParams>,
// ) -> Response {
//     let tag_id = match params.tag_id {
//         Some(v) => v,
//         None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
//     };
//
//     let mut transaction = match state.db.begin().await {
//         Ok(transaction) => transaction,
//         Err(err) => {
//             error!("{:#?}", err);
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     };
//
//     let tag = match TagRepositoryImpl::find(&mut transaction, tag_id).await {
//         Ok(tag) => tag,
//         Err(err) => {
//             error!("{:#?}", err);
//             if let sqlx::error::Error::RowNotFound = err {
//                 return (StatusCode::NOT_FOUND, "Tag not found").into_response();
//             }
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     };
//
//     let tag = match TagRepositoryImpl::delete(&mut transaction, tag.id).await {
//         Ok(tag) => tag,
//         Err(err) => {
//             error!("{:#?}", err);
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     };
//
//     match transaction.commit().await {
//         Ok(_) => (StatusCode::OK, format!("Deleted tag id: {}", tag.id)).into_response(),
//         Err(err) => {
//             error!("{:#?}", err);
//             return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
//         }
//     }
// }
