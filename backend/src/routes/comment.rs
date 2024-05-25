use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::Deserialize;
use serde_json::json;
use shared::{
    models::{comment_model::Comment, enums::CommentableType},
    resource_proto::{
        self, comment_service_client::CommentServiceClient, CreateCommentRequest,
        DeleteCommentRequest, GetCommentsRequest, QueryParams, UpdateCommentRequest,
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
pub struct CommentsQueryParams {
    id: String,
    r#type: CommentableType,
    user_id: Option<String>,
}

pub async fn get_comments(
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Query(query): Query<CommentsQueryParams>,
    Query(pagination): Query<Pagination>,
) -> Response {
    info!("Get Comments Request {:#?} {:#?}", query, pagination);

    match CommentServiceClient::new(channel)
        .get_comments(GetCommentsRequest {
            user_id: query.user_id,
            target_id: query.id,
            r#type: resource_proto::CommentableType::from(query.r#type) as i32,
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
                Json(json!(ResultPaging::<Comment> {
                    total: res.total,
                    pagination: Metadata::new(res.total, pagination.per_page, pagination.page),
                    items: res
                        .comments
                        .iter()
                        .map(|comment| Comment::from(comment))
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

// pub async fn get_comment(
//     Extension(user): Extension<AccessTokenPayload>,
//     State(state): State<AppState>,
//     Path(params): Path<PathParams>,
// ) -> Response {
//     let comment_id = match params.comment_id {
//         Some(v) => v,
//         None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
//     };
// }

#[derive(Debug, Deserialize)]
pub struct PostCommentRequestBody {
    content: String,
}

pub async fn post_comment(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Query(query): Query<CommentsQueryParams>,
    Json(payload): Json<PostCommentRequestBody>,
) -> Response {
    info!(
        "Post Comment Request {:#?} {:#?} {:#?}",
        user, query, payload
    );

    match CommentServiceClient::new(channel)
        .create_comment(CreateCommentRequest {
            user_id: user.user_id,
            target_id: query.id,
            r#type: resource_proto::CommentableType::from(query.r#type) as i32,
            content: payload.content,
        })
        .await
    {
        Ok(res) => (
            StatusCode::CREATED,
            Json(json!(Comment::from(res.get_ref()))),
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
pub struct PatchCommentRequestBody {
    pub content: Option<String>,
}

pub async fn patch_comment(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchCommentRequestBody>,
) -> Response {
    let comment_id = match params.comment_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    info!(
        "Patch Comment Request {:#?} {:#?} {:#?}",
        user, comment_id, payload
    );

    match CommentServiceClient::new(channel)
        .update_comment(UpdateCommentRequest {
            comment_id,
            user_id: user.user_id,
            content: payload.content,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(Comment::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}

pub async fn delete_comment(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let comment_id = match params.comment_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!("Delete Comment Request {:#?} {:#?}", user, comment_id);
    match CommentServiceClient::new(channel)
        .delete_comment(DeleteCommentRequest {
            comment_id,
            user_id: user.user_id,
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
