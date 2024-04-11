use axum::{
    async_trait,
    routing::{get, patch, post, put},
    Router,
};
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

use crate::{
    models::article_model::{AddAuthor, CreateArticle, DeleteAuthor, UpdateArticle},
    repositories::{article_repository::ArticleRepository, user_repository::UserRepository},
    AppState,
};

pub fn router() -> Router<Arc<AppState>> {
    // recs
    Router::new()
        .route("/articles", get(get_articles).post(post_article))
        .route("/articles/:article_id", get(get_article))
        .route("/articles/:article_id/authors", get(get_authors))
        .route(
            "/articles/:article_id/edit",
            patch(patch_article).delete(delete_article),
        )
        .route(
            "/articles/:article_id/edit/authors/:user_id",
            put(put_author).delete(delete_author),
        )
        .route("/users/:user_id/articles", get(get_articles_by_user))
}

pub async fn get_articles(State(state): State<Arc<AppState>>) -> Response {
    let response = state.repository.articles.find_all().await;
    match response {
        Ok(article) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(error.to_string())),
        )
            .into_response(),
    }
}

pub async fn get_articles_by_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Response {
    let response = state.repository.articles.find_by_authors(&[user_id]).await;
    match response {
        Ok(articles) => (StatusCode::OK, Json(json!(articles))).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(error.to_string())),
        )
            .into_response(),
    }
}

pub async fn get_article(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Response {
    let response = state.repository.articles.find_by_id(article_id).await;
    match response {
        Ok(article) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(error) => {
            if let sqlx::error::Error::RowNotFound = error {
                return (StatusCode::NOT_FOUND, "Article not found").into_response();
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response()
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
    let create_article = CreateArticle {
        title: payload.title,
        user_id: payload.user_id,
    };
    let response = state.repository.articles.create(&create_article).await;
    match response {
        Ok(article) => (StatusCode::CREATED, Json(json!(article))).into_response(),
        Err(error) => {
            if let Some(database_error) = error.as_database_error() {
                if let Some(constraint) = database_error.constraint() {
                    if constraint == "authors_author_id_fkey" {
                        return (StatusCode::BAD_REQUEST, "User not found").into_response();
                    }
                    if constraint == "articles_slug_key" {
                        return (StatusCode::BAD_REQUEST, "Retry").into_response();
                    }
                }
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response()
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
    let udpate_article = UpdateArticle {
        id: article_id,
        title: payload.title,
    };
    let response = state.repository.articles.update(&udpate_article).await;
    match response {
        Ok(article) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(error) => {
            if let sqlx::error::Error::RowNotFound = error {
                return (StatusCode::NOT_FOUND, "Article not found").into_response();
            }
            if let Some(database_error) = error.as_database_error() {
                if let Some(constraint) = database_error.constraint() {
                    if constraint == "articles_slug_key" {
                        return (StatusCode::BAD_REQUEST, "Retry").into_response();
                    }
                }
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response()
        }
    }
}

pub async fn delete_article(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Response {
    let response = state.repository.articles.delete(article_id).await;
    match response {
        Ok(_) => (StatusCode::OK, format!("Deleted article: {article_id}")).into_response(),
        Err(error) => {
            if let sqlx::error::Error::RowNotFound = error {
                return (StatusCode::NOT_FOUND, "Article not found").into_response();
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response()
        }
    }
}

pub async fn get_authors(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Response {
    let response = state.repository.articles.find_by_id(article_id).await;

    if let Err(error) = response {
        if let sqlx::error::Error::RowNotFound = error {
            return (StatusCode::NOT_FOUND, "Article not found").into_response();
        }
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(error.to_string())),
        )
            .into_response();
    }

    let response = state.repository.articles.get_authors(article_id).await;

    match response {
        Ok(users) => (StatusCode::OK, Json(json!(users))).into_response(),
        Err(error) => {
            if let sqlx::error::Error::RowNotFound = error {
                return (StatusCode::NOT_FOUND, "Article not found").into_response();
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response()
        }
    }
}

pub async fn put_author(
    State(state): State<Arc<AppState>>,
    Path((article_id, user_id)): Path<(i32, i32)>,
) -> Response {
    let response = state.repository.articles.find_by_id(article_id).await;

    if let Err(error) = response {
        if let sqlx::error::Error::RowNotFound = error {
            return (StatusCode::NOT_FOUND, "Article not found").into_response();
        }
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(error.to_string())),
        )
            .into_response();
    }

    let response = state.repository.users.find_by_id(user_id).await;

    if let Err(error) = response {
        if let sqlx::error::Error::RowNotFound = error {
            return (StatusCode::NOT_FOUND, "User not found").into_response();
        }
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(error.to_string())),
        )
            .into_response();
    }

    let add_author = AddAuthor {
        user_id,
        article_id,
    };

    let response = state.repository.articles.add_author(&add_author).await;

    match response {
        Ok(_) => (
            StatusCode::OK,
            format!("Author {} added to {}", user_id, article_id),
        )
            .into_response(),
        Err(error) => {
            if let sqlx::error::Error::RowNotFound = error {
                return (StatusCode::NOT_FOUND, "Article not found").into_response();
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(error.to_string())),
            )
                .into_response()
        }
    }
}

pub async fn delete_author(
    State(state): State<Arc<AppState>>,
    Path((article_id, user_id)): Path<(i32, i32)>,
) -> Response {
    let response = state.repository.articles.find_by_id(article_id).await;

    if let Err(error) = response {
        if let sqlx::error::Error::RowNotFound = error {
            return (StatusCode::NOT_FOUND, "Article not found").into_response();
        }
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(error.to_string())),
        )
            .into_response();
    }

    let response = state.repository.users.find_by_id(user_id).await;

    if let Err(error) = response {
        if let sqlx::error::Error::RowNotFound = error {
            return (StatusCode::NOT_FOUND, "User not found").into_response();
        }
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(error.to_string())),
        )
            .into_response();
    }

    let delete_author = DeleteAuthor {
        user_id,
        article_id,
    };

    let response = state
        .repository
        .articles
        .delete_author(&delete_author)
        .await;
    match response {
        Ok(_) => (
            StatusCode::OK,
            format!("Author {} deleted to {}", user_id, article_id),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}
