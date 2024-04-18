use std::sync::Arc;

use axum::{
    http::StatusCode,
    routing::{get, patch, post, put},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;

use crate::application::AppState;

use self::{
    admin::health_checker,
    article::{
        delete_article, delete_author, get_article, get_articles, get_articles_by_user,
        get_authors, patch_article, post_article, put_author,
    },
    comment::{delete_comment, get_comments, patch_comment, post_comment},
    list::{
        delete_list, delete_list_article, get_list, get_list_articles, get_list_by_user, get_lists,
        patch_list, post_list, put_list_article,
    },
    series::{
        delete_series, delete_series_article, get_series, get_series_articles, get_series_by_user,
        patch_series, post_series, put_series_article,
    },
    tags::{delete_tag, get_tag, get_tags, patch_tag, post_tag},
    user::{delete_user, get_user, get_users, patch_user, post_user},
};

pub mod admin;
pub mod article;
pub mod auth;
pub mod comment;
pub mod list;
pub mod series;
pub mod tags;
pub mod user;

pub fn router() -> Router<Arc<AppState>> {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    Router::new()
        .nest(
            "/api",
            Router::new()
                .merge(auth::router())
                .nest(
                    "/admin",
                    Router::new()
                        .route("/tags", get(get_tags).post(post_tag))
                        .route(
                            "/tags/:tag_id",
                            get(get_tag).patch(patch_tag).delete(delete_tag),
                        )
                        .route("/healthchecker", get(health_checker))
                        .route("/metrics", get(|| async move { metric_handle.render() })),
                )
                .nest(
                    "/users",
                    Router::new()
                        .route("/", get(get_users).post(post_user))
                        .route(
                            "/:user_id",
                            get(get_user).patch(patch_user).delete(delete_user),
                        )
                        .route("/:user_id/lists", get(get_list_by_user))
                        .route("/:user_id/articles", get(get_articles_by_user))
                        .route("/:user_id/series", get(get_series_by_user)),
                )
                .nest(
                    "/articles",
                    Router::new()
                        .route("/", get(get_articles).post(post_article))
                        .route("/:article_id", get(get_article))
                        .route("/:article_id/authors", get(get_authors))
                        .route(
                            "/:article_id/edit",
                            patch(patch_article).delete(delete_article),
                        )
                        .route(
                            "/:article_id/edit/authors/:user_id",
                            put(put_author).delete(delete_author),
                        )
                        .route(
                            "/:article_id/comments",
                            post(post_comment).get(get_comments),
                        )
                        .route(
                            "/:article_id/comments/:comment_id",
                            patch(patch_comment).delete(delete_comment),
                        )
                        .route(
                            "/:article_id/edit/tags",
                            put(StatusCode::NOT_IMPLEMENTED).delete(StatusCode::NOT_IMPLEMENTED),
                        ),
                )
                .nest(
                    "/lists",
                    Router::new()
                        .route("/", get(get_lists).post(post_list))
                        .route(
                            "/:list_id",
                            get(get_list).patch(patch_list).delete(delete_list),
                        )
                        .route("/:list_id/articles", get(get_list_articles))
                        .route(
                            "/:list_id/articles/:article_id",
                            put(put_list_article).delete(delete_list_article),
                        ),
                )
                .nest(
                    "/series",
                    Router::new()
                        .route("/", get(get_series).post(post_series))
                        .route("/:series_id", patch(patch_series).delete(delete_series))
                        .route("/:series_id/articles", get(get_series_articles))
                        .route(
                            "/:series_id/articles/:article_id",
                            put(put_series_article).delete(delete_series_article),
                        ),
                ),
        )
        .layer(prometheus_layer)
}
