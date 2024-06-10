use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Extension, Json,
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use serde::Deserialize;
use serde_json::json;
use shared::{
    comment::{
        comment_service_client::CommentServiceClient, CreateRequest, DeleteRequest, GetAllRequest,
        UpdateRequest,
    },
    common,
    models::{
        comment_model::{Comment, FullComment},
        enums::CommentableType,
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
pub struct CommentsQueryParams {
    query: Option<String>,
}

pub async fn get_comments(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Query(query): Query<CommentsQueryParams>,
    Query(cursor): Query<CursorPagination>,
    Path(params): Path<PathParams>,
) -> Response {
    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();

    let target_id;
    let r#type: CommentableType;

    if let Some(article_id) = params.article_id {
        target_id = article_id;
        r#type = CommentableType::Article;
    } else if let Some(series_id) = params.series_id {
        target_id = series_id;
        r#type = CommentableType::Series;
    } else if let Some(list_id) = params.list_id {
        target_id = list_id;
        r#type = CommentableType::List;
    } else {
        return (StatusCode::BAD_REQUEST).into_response();
    }

    info!("Get Comments Request {:?} {:?}", query, cursor);

    match CommentServiceClient::new(channel)
        .get_all(GetAllRequest {
            query: query.query,
            target_id,
            r#type: common::CommentableType::from(r#type) as i32,
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
                Json(json!(ResultPaging::<FullComment> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res
                        .comments
                        .iter()
                        .map(|comment| FullComment::from(comment))
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

#[derive(Debug, Deserialize)]
pub struct PostCommentRequestBody {
    content: String,
}

pub async fn post_comment(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Query(query): Query<CommentsQueryParams>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PostCommentRequestBody>,
) -> Response {
    let target_id;
    let r#type: CommentableType;

    if let Some(article_id) = params.article_id {
        target_id = article_id;
        r#type = CommentableType::Article;
    } else if let Some(series_id) = params.series_id {
        target_id = series_id;
        r#type = CommentableType::Series;
    } else if let Some(list_id) = params.list_id {
        target_id = list_id;
        r#type = CommentableType::List;
    } else {
        return (StatusCode::BAD_REQUEST).into_response();
    }

    info!("Post Comment Request {:?} {:?} {:?}", user, query, payload);

    match CommentServiceClient::new(channel)
        .create(CreateRequest {
            user_id: user.user_id,
            target_id,
            r#type: common::CommentableType::from(r#type) as i32,
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
            error!("{:?}", err);
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
        "Patch Comment Request {:?} {:?} {:?}",
        user, comment_id, payload
    );

    match CommentServiceClient::new(channel)
        .update(UpdateRequest {
            comment_id,
            user_id: user.user_id,
            content: payload.content,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(Comment::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:?}", err);
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
    info!("Delete Comment Request {:?} {:?}", user, comment_id);
    match CommentServiceClient::new(channel)
        .delete(DeleteRequest {
            comment_id,
            user_id: user.user_id,
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
