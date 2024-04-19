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
    models::article_model::{AddAuthor, CreateArticle, DeleteAuthor, UpdateArticle},
    repositories::article_repository::{ArticleRepository, ArticleRepositoryImpl},
};

pub async fn get_articles(State(state): State<Arc<AppState>>) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let articles = match ArticleRepositoryImpl::find_all(&mut transaction).await {
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

pub async fn get_articles_by_user(
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
    let articles = match ArticleRepositoryImpl::find_by_authors(&mut transaction, &[user_id]).await
    {
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

pub async fn get_article(
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
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostArticleRequestBody {
    title: String,
    user_id: i32,
}

pub async fn post_article(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PostArticleRequestBody>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let create_article = CreateArticle {
        title: payload.title,
        user_id: payload.user_id,
    };
    let article = match ArticleRepositoryImpl::create(&mut transaction, &create_article).await {
        Ok(article) => article,
        Err(err) => {
            error!("{:#?}", err);
            if let Some(database_error) = err.as_database_error() {
                if let Some(constraint) = database_error.constraint() {
                    if constraint == "authors_author_id_fkey" {
                        return (StatusCode::BAD_REQUEST, "User not found").into_response();
                    }
                    if constraint == "articles_slug_key" {
                        return (StatusCode::BAD_REQUEST, "Retry").into_response();
                    }
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchArticleRequestBody {
    pub title: Option<String>,
}

pub async fn patch_article(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
    Json(payload): Json<PatchArticleRequestBody>,
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
    let udpate_article = UpdateArticle {
        id: article.id,
        title: payload.title,
    };
    let article = match ArticleRepositoryImpl::update(&mut transaction, &udpate_article).await {
        Ok(article) => article,
        Err(err) => {
            error!("{:#?}", err);
            if let Some(database_error) = err.as_database_error() {
                if let Some(constraint) = database_error.constraint() {
                    if constraint == "articles_slug_key" {
                        return (StatusCode::BAD_REQUEST, "Retry").into_response();
                    }
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn delete_article(
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
    let article = match ArticleRepositoryImpl::delete(&mut transaction, article.id).await {
        Ok(article) => article,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, format!("Deleted article: {}", article.id)).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn get_authors(
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
    let authors = match ArticleRepositoryImpl::get_authors(&mut transaction, article_id).await {
        Ok(users) => users,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (StatusCode::OK, Json(json!(authors))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn put_author(
    State(state): State<Arc<AppState>>,
    Path((article_id, user_id)): Path<(i32, i32)>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let add_author = AddAuthor {
        user_id,
        article_id,
    };
    let authors = match ArticleRepositoryImpl::add_author(&mut transaction, &add_author).await {
        Ok(users) => users,
        Err(err) => {
            error!("{:#?}", err);
            // if let sqlx::error::Error::RowNotFound = err {
            //     return (StatusCode::NOT_FOUND, "Article not found").into_response();
            // }
            if let Some(database_error) = err.as_database_error() {
                if let Some(constraint) = database_error.constraint() {
                    if constraint == "authors_author_id_fkey" {
                        return (StatusCode::BAD_REQUEST, "User does not exist").into_response();
                    }
                    if constraint == "authors_article_id_fkey" {
                        return (StatusCode::BAD_REQUEST, "Article does not exist").into_response();
                    }
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (
            StatusCode::OK,
            format!("Author {} added to {}", authors.1, authors.0),
        )
            .into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}

pub async fn delete_author(
    State(state): State<Arc<AppState>>,
    Path((article_id, user_id)): Path<(i32, i32)>,
) -> Response {
    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    let delete_author = DeleteAuthor {
        user_id,
        article_id,
    };

    let authors = match ArticleRepositoryImpl::delete_author(&mut transaction, &delete_author).await
    {
        Ok(users) => users,
        Err(err) => {
            error!("{:#?}", err);
            // if let sqlx::error::Error::RowNotFound = err {
            //     return (StatusCode::NOT_FOUND, "Article not found").into_response();
            // }
            if let Some(database_error) = err.as_database_error() {
                if let Some(constraint) = database_error.constraint() {
                    if constraint == "authors_author_id_fkey" {
                        return (StatusCode::BAD_REQUEST, "User does not exist").into_response();
                    }
                    if constraint == "authors_article_id_fkey" {
                        return (StatusCode::BAD_REQUEST, "Article does not exist").into_response();
                    }
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match transaction.commit().await {
        Ok(_) => (
            StatusCode::OK,
            format!("Author {} deleted from {}", authors.1, authors.0),
        )
            .into_response(),
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    }
}
