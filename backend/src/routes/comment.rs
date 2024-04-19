use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    application::AppState,
    models::comment_model::{CreateComment, UpdateComment},
    repositories::{
        article_repository::{ArticleRepository, ArticleRepositoryImpl},
        comment_repository::{CommentRepository, CommentRepositoryImpl},
    },
};

pub async fn get_comments(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let article = match ArticleRepositoryImpl::find(&mut transaction, article_id).await {
        Ok(article) => article,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "Article not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let comments =
        match CommentRepositoryImpl::find_all_by_article_id(&mut transaction, article.id).await {
            Ok(comments) => comments,
            Err(err) => {
                error!("{:#?}", err);

                return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
            }
        };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(comments))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn get_comment(
    State(state): State<Arc<AppState>>,
    Path((article_id, comment_id)): Path<(i32, i32)>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let _article = match ArticleRepositoryImpl::find(&mut transaction, article_id).await {
        Ok(article) => article,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "Article not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let comment = match CommentRepositoryImpl::find(&mut transaction, comment_id).await {
        Ok(comment) => comment,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "Comment not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(comment))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostCommentRequestBody {
    user_id: i32,
    content: String,
}

pub async fn post_comment(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
    Json(payload): Json<PostCommentRequestBody>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let comment = match CommentRepositoryImpl::create(
        &mut transaction,
        &CreateComment {
            article_id,
            user_id: payload.user_id,
            content: payload.content,
        },
    )
    .await
    {
        Ok(comment) => comment,
        Err(err) => {
            error!("{:#?}", err);
            if let Some(database_error) = err.as_database_error() {
                if let Some(constraint) = database_error.constraint() {
                    if constraint == "comments_commenter_id_fkey" {
                        return (StatusCode::BAD_REQUEST, "User not found").into_response();
                    }
                    if constraint == "comments_article_id_fkey" {
                        return (StatusCode::BAD_REQUEST, "Article not found").into_response();
                    }
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(comment))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchCommentRequestBody {
    pub user_id: i32,
    pub content: Option<String>,
}

pub async fn patch_comment(
    State(state): State<Arc<AppState>>,
    Path((article_id, comment_id)): Path<(i32, i32)>,
    Json(payload): Json<PatchCommentRequestBody>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let article = match ArticleRepositoryImpl::find(&mut transaction, article_id).await {
        Ok(article) => article,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "Article not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let comment = match CommentRepositoryImpl::update(
        &mut transaction,
        &UpdateComment {
            id: comment_id,
            article_id: article.id,
            user_id: payload.user_id,
            content: payload.content,
        },
    )
    .await
    {
        Ok(comment) => comment,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(comment))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn delete_comment(
    State(state): State<Arc<AppState>>,
    Path((article_id, comment_id)): Path<(i32, i32)>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let article = match ArticleRepositoryImpl::find(&mut transaction, article_id).await {
        Ok(article) => article,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "Article not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let comment = match CommentRepositoryImpl::find(&mut transaction, comment_id).await {
        Ok(comment) => comment,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "Comment not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let comment =
        match CommentRepositoryImpl::delete(&mut transaction, comment.id, article.id).await {
            Ok(comment) => comment,
            Err(err) => {
                error!("{:#?}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
            }
        };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, format!("Deleted comment: {}", comment.id)).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}
