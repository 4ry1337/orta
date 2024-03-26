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
    repositories::article_repository::ArticleRepository,
    AppState,
};

pub async fn get_articles(State(state): State<Arc<AppState>>) -> Response {
    let response = state.repository.article.find_all().await;
    match response {
        Ok(article) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn get_articles_by_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Response {
    let response = state.repository.article.find_by_authors(&[user_id]).await;
    match response {
        Ok(articles) => (StatusCode::OK, Json(json!(articles))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn get_article(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Response {
    let response = state.repository.article.find_by_id(article_id).await;
    match response {
        Ok(article) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
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
    let response = state.repository.article.create(&create_article).await;
    match response {
        Ok(article) => (StatusCode::CREATED, Json(json!(article))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
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
    let response = state.repository.article.update(&udpate_article).await;
    match response {
        Ok(article) => (StatusCode::OK, Json(json!(article))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn delete_article(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Response {
    let response = state.repository.article.delete(article_id).await;
    match response {
        Ok(()) => (StatusCode::OK, format!("Deleted article: {article_id}")).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn get_authors(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<i32>,
) -> Response {
    let response = state.repository.article.get_authors(article_id).await;
    match response {
        Ok(users) => (StatusCode::OK, Json(json!(users))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct PostAuthorRequestBody {
    user_id: i32,
}

pub async fn post_author(
    Path(article_id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PostAuthorRequestBody>,
) -> Response {
    let add_author = AddAuthor {
        user_id: payload.user_id,
        article_id,
    };
    let response = state.repository.article.add_author(&add_author).await;
    match response {
        Ok(()) => (
            StatusCode::OK,
            format!("Author {} added to {}", payload.user_id, article_id),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteAuthorRequestBody {
    user_id: i32,
}

pub async fn delete_author(
    Path(article_id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeleteAuthorRequestBody>,
) -> Response {
    let delete_author = DeleteAuthor {
        user_id: payload.user_id,
        article_id,
    };
    let response = state.repository.article.delete_author(&delete_author).await;
    match response {
        Ok(()) => (
            StatusCode::OK,
            format!("Author {} deleted to {}", payload.user_id, article_id),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}
