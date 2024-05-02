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
use shared::{models::prelude::*, repositories::prelude::*};
use tracing::error;

use crate::{application::AppState, utils::params::PathParams};

pub async fn get_lists(State(state): State<Arc<AppState>>) -> Response {}

pub async fn get_list(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

#[derive(Debug, Deserialize)]
pub struct PostListRequestBody {
    pub user_id: i32,
    pub label: String,
    pub image: Option<String>,
    pub visibility: Visibility,
}

pub async fn post_list(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PostListRequestBody>,
) -> Response {
}

#[derive(Debug, Deserialize)]
pub struct PatchListRequestBody {
    pub label: Option<String>,
    pub image: Option<String>,
    pub visibility: Option<Visibility>,
}

pub async fn patch_list(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
    Json(payload): Json<PatchListRequestBody>,
) -> Response {
    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

pub async fn delete_list(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
}

pub async fn get_list_by_user(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let user_id = match params.user_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let user = match UserRepositoryImpl::find(&mut transaction, user_id).await {
        Ok(user) => user,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "User not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let lists = match ListRepositoryImpl::find_by_user(&mut transaction, user.id).await {
        Ok(lists) => lists,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(lists))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn get_list_articles(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let list = match ListRepositoryImpl::find(&mut transaction, list_id).await {
        Ok(list) => list,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "List not found").into_response();
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let articles = match ListRepositoryImpl::find_articles(&mut transaction, list.id).await {
        Ok(articles) => articles,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(articles))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn put_list_article(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let list = match ListRepositoryImpl::find(&mut transaction, list_id).await {
        Ok(list) => list,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "List not found").into_response();
            }
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

    let reponse = match ListRepositoryImpl::add_article(&mut transaction, list.id, article.id).await
    {
        Ok(reponse) => reponse,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    match transaction.commit().await {
        Ok(()) => (
            StatusCode::OK,
            format!("Article {} added to List {}", reponse.1, reponse.0),
        )
            .into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn delete_list_article(
    State(state): State<Arc<AppState>>,
    Path(params): Path<PathParams>,
) -> Response {
    let article_id = match params.article_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let list_id = match params.list_id {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };

    let list = match ListRepositoryImpl::find(&mut transaction, list_id).await {
        Ok(list) => list,
        Err(err) => {
            error!("{:#?}", err);
            if let sqlx::error::Error::RowNotFound = err {
                return (StatusCode::NOT_FOUND, "List not found").into_response();
            }
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

    let reponse =
        match ListRepositoryImpl::remove_article(&mut transaction, list.id, article.id).await {
            Ok(reponse) => reponse,
            Err(err) => {
                error!("{:#?}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
            }
        };

    match transaction.commit().await {
        Ok(()) => (
            StatusCode::OK,
            format!("Article {} removed from List {}", reponse.1, reponse.0),
        )
            .into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}
