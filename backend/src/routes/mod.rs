use axum::{
    handler::Handler,
    middleware,
    routing::{get, patch, put},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;

use crate::{application::AppState, middlewares::auth::auth_middleware};

use self::{
    admin::health_checker,
    article::{
        delete_article, delete_author, get_article, get_articles, patch_article, post_article,
        put_author,
    },
    comment::{delete_comment, get_comments, patch_comment, post_comment},
    list::{
        delete_list, delete_list_article, get_list, get_lists, patch_list, post_list,
        put_list_article,
    },
    series::{
        delete_series, delete_series_article, get_serieses, patch_series, post_series,
        put_series_article,
    },
    user::{get_user, get_users, patch_user},
};

pub mod admin;
pub mod article;
pub mod auth;
pub mod comment;
pub mod list;
pub mod series;
pub mod tags;
pub mod user;

pub fn router(state: AppState) -> Router<AppState> {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    Router::new()
        .nest(
            "/api",
            Router::new()
                .merge(auth::router())
                .nest(
                    "/admin",
                    Router::new()
                        .route("/healthchecker", get(health_checker))
                        .route("/metrics", get(|| async move { metric_handle.render() }))
                        .layer(middleware::from_fn_with_state(
                            state.clone(),
                            auth_middleware,
                        )),
                )
                .nest(
                    "/users",
                    Router::new().route("/", get(get_users)).route(
                        "/:username",
                        get(get_user).patch(patch_user.layer(middleware::from_fn_with_state(
                            state.clone(),
                            auth_middleware,
                        ))),
                    ),
                )
                .nest(
                    "/articles",
                    Router::new()
                        .route(
                            "/",
                            get(get_articles).post(post_article.layer(
                                middleware::from_fn_with_state(state.clone(), auth_middleware),
                            )),
                        )
                        .nest(
                            "/slug/:article_slug",
                            Router::new().route("/", get(get_article)),
                        )
                        .nest(
                            "/id/:article_id",
                            Router::new()
                                .route("/", patch(patch_article).delete(delete_article))
                                .route("/authors", put(put_author).delete(delete_author))
                                // .route(
                                //     "/tags",
                                //     put(StatusCode::NOT_IMPLEMENTED)
                                //         .delete(StatusCode::NOT_IMPLEMENTED),
                                // )
                                .layer(middleware::from_fn_with_state(
                                    state.clone(),
                                    auth_middleware,
                                )),
                        ),
                )
                .nest(
                    "/lists",
                    Router::new()
                        .route(
                            "/",
                            get(get_lists).post(post_list.layer(middleware::from_fn_with_state(
                                state.clone(),
                                auth_middleware,
                            ))),
                        )
                        .nest("/slug/:list_slug", Router::new().route("/", get(get_list)))
                        .nest(
                            "/id/:list_id",
                            Router::new()
                                .route("/", patch(patch_list).delete(delete_list))
                                .route(
                                    "/articles",
                                    put(put_list_article).delete(delete_list_article),
                                )
                                .layer(middleware::from_fn_with_state(
                                    state.clone(),
                                    auth_middleware,
                                )),
                        ),
                )
                .nest(
                    "/series",
                    Router::new()
                        .route(
                            "/",
                            get(get_serieses).post(post_series.layer(
                                middleware::from_fn_with_state(state.clone(), auth_middleware),
                            )),
                        )
                        .nest(
                            "/id/:series_id",
                            Router::new()
                                .route("/", patch(patch_series).delete(delete_series))
                                .route(
                                    "/articles",
                                    put(put_series_article).delete(delete_series_article),
                                )
                                .layer(middleware::from_fn_with_state(
                                    state.clone(),
                                    auth_middleware,
                                )),
                        ),
                )
                .nest(
                    "/comments",
                    Router::new()
                        .route(
                            "/",
                            get(get_comments).post(post_comment.layer(
                                middleware::from_fn_with_state(state.clone(), auth_middleware),
                            )),
                        )
                        .route(
                            "/:comment_id",
                            patch(patch_comment).delete(delete_comment).layer(
                                middleware::from_fn_with_state(state.clone(), auth_middleware),
                            ),
                        ),
                ),
        )
        .layer(prometheus_layer)
}
