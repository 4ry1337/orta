use axum::{
    extract::{DefaultBodyLimit, State},
    handler::Handler,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, patch, post, put},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;
use tracing::info;

use crate::{
    application::AppState,
    middlewares::{
        auth::auth_middleware,
        service_client::{
            auth_service_middleware, resource_service_middleware, storage_service_middleware,
        },
    },
};

use self::{
    admin::health_checker,
    article::{
        delete_article, delete_author, get_article, get_articles, patch_article, post_article,
        put_author,
    },
    assets::{get_asset, post_asset},
    comment::{delete_comment, get_comments, patch_comment, post_comment},
    list::{
        delete_list, delete_list_article, get_list, get_lists, patch_list, post_list,
        put_list_article,
    },
    series::{
        delete_series, delete_series_article, get_series, get_serieses, patch_series, post_series,
        put_series_article,
    },
    user::{get_user, get_users, patch_user},
};

pub mod admin;
pub mod article;
pub mod assets;
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
                .nest(
                    "/assets",
                    Router::new()
                        .route("/", post(post_asset))
                        .route("/:asset_name", get(get_asset))
                        .layer(middleware::from_fn(storage_service_middleware)),
                )
                .merge(
                    auth::router(state.clone()).layer(middleware::from_fn_with_state(
                        state.clone(),
                        auth_service_middleware,
                    )),
                )
                .nest(
                    "/admin",
                    Router::new()
                        .route("/healthchecker", get(health_checker))
                        .route("/metrics", get(|| async move { metric_handle.render() }))
                        .layer(middleware::from_fn(auth_middleware)),
                )
                .nest(
                    "/users",
                    Router::new()
                        .route("/", get(get_users))
                        .route(
                            "/:username",
                            get(get_user).patch(patch_user.layer(middleware::from_fn_with_state(
                                state.clone(),
                                auth_middleware,
                            ))),
                        )
                        .layer(middleware::from_fn(resource_service_middleware)),
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
                            "/:article_id",
                            Router::new()
                                .route(
                                    "/",
                                    get(get_article)
                                        .patch(patch_article.layer(middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
                                        )))
                                        .delete(delete_article.layer(
                                            middleware::from_fn_with_state(
                                                state.clone(),
                                                auth_middleware,
                                            ),
                                        )),
                                )
                                .route(
                                    "/authors",
                                    put(put_author).delete(delete_author).layer(
                                        middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
                                        ),
                                    ),
                                ),
                        )
                        .layer(middleware::from_fn(resource_service_middleware)),
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
                        .nest(
                            "/:list_id",
                            Router::new()
                                .route(
                                    "/",
                                    get(get_list)
                                        .patch(patch_list.layer(middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
                                        )))
                                        .delete(delete_list.layer(middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
                                        ))),
                                )
                                .route(
                                    "/articles",
                                    put(put_list_article).delete(delete_list_article).layer(
                                        middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
                                        ),
                                    ),
                                ),
                        )
                        .layer(middleware::from_fn(resource_service_middleware)),
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
                            "/:series_id",
                            Router::new()
                                .route(
                                    "/",
                                    get(get_series)
                                        .patch(patch_series.layer(middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
                                        )))
                                        .delete(delete_series.layer(
                                            middleware::from_fn_with_state(
                                                state.clone(),
                                                auth_middleware,
                                            ),
                                        )),
                                )
                                .route(
                                    "/articles",
                                    put(put_series_article).delete(delete_series_article).layer(
                                        middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
                                        ),
                                    ),
                                ),
                        )
                        .layer(middleware::from_fn(resource_service_middleware)),
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
                        )
                        .layer(middleware::from_fn(resource_service_middleware)),
                )
                .route("/notify", get(test_notification)),
        )
        .layer(DefaultBodyLimit::max(1024 * 1024 * 50))
        .layer(prometheus_layer)
}

pub async fn test_notification(State(state): State<AppState>) -> Response {
    info!("test notification faired");

    (StatusCode::OK, "test notification faired").into_response()
}
