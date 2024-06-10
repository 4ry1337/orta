use crate::{
    application::AppState,
    middlewares::{
        auth::auth_middleware,
        service_client::{
            auth_service_middleware, resource_service_middleware, storage_service_middleware,
        },
    },
};
use article::put_article_tags;
use auth::get_session;
use axum::{
    extract::DefaultBodyLimit,
    handler::Handler,
    middleware,
    routing::{get, patch, post, put},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;
use list::get_list_articles;
use series::{get_series_articles, patch_series_article};
use tags::get_tags;
use user::{get_user_articles, get_user_drafts};

use self::{
    admin::health_checker,
    article::{
        delete_article, delete_author, edit_article, get_article, get_history, like_article,
        patch_article, post_article, publish, put_author, search_articles, unlike_article,
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
    user::{
        follow, get_feed, get_followers, get_following, get_user, get_user_lists,
        get_user_serieses, get_users, patch_user, unfollow,
    },
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
                    "/me",
                    Router::new()
                        .route("/", get(get_session))
                        .route("/feed", get(get_feed))
                        .route("/drafts", get(get_user_drafts))
                        .layer(middleware::from_fn_with_state(
                            state.clone(),
                            auth_middleware,
                        ))
                        .layer(middleware::from_fn(resource_service_middleware)),
                )
                .nest(
                    "/users",
                    Router::new()
                        .route("/", get(get_users))
                        .nest(
                            "/:username",
                            Router::new()
                                .route(
                                    "/",
                                    get(get_user).patch(patch_user.layer(
                                        middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
                                        ),
                                    )),
                                )
                                .route("/articles", get(get_user_articles))
                                .route("/series", get(get_user_serieses))
                                .route("/lists", get(get_user_lists))
                                // .route("/interests", get(get_user_articles))
                                .route(
                                    "/follow",
                                    put(follow).delete(unfollow).layer(
                                        middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
                                        ),
                                    ),
                                )
                                .route("/followers", get(get_followers))
                                .route("/following", get(get_following)),
                        )
                        .layer(middleware::from_fn(resource_service_middleware)),
                )
                .nest(
                    "/articles",
                    Router::new()
                        .route(
                            "/",
                            get(search_articles).post(post_article.layer(
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
                                .nest(
                                    "/comments",
                                    Router::new()
                                        .route(
                                            "/",
                                            get(get_comments).post(post_comment.layer(
                                                middleware::from_fn_with_state(
                                                    state.clone(),
                                                    auth_middleware,
                                                ),
                                            )),
                                        )
                                        .route(
                                            "/:comment_id",
                                            patch(patch_comment).delete(delete_comment).layer(
                                                middleware::from_fn_with_state(
                                                    state.clone(),
                                                    auth_middleware,
                                                ),
                                            ),
                                        ),
                                )
                                .route(
                                    "/like",
                                    put(like_article).delete(unlike_article).layer(
                                        middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
                                        ),
                                    ),
                                )
                                .route(
                                    "/history",
                                    get(get_history).layer(middleware::from_fn_with_state(
                                        state.clone(),
                                        auth_middleware,
                                    )),
                                )
                                .route(
                                    "/publish",
                                    patch(publish).layer(middleware::from_fn_with_state(
                                        state.clone(),
                                        auth_middleware,
                                    )),
                                )
                                .nest(
                                    "/edit",
                                    Router::new()
                                        .route("/", patch(edit_article))
                                        .route("/tags", put(put_article_tags))
                                        .layer(middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
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
                    "/tags",
                    Router::new()
                        .route("/", get(get_tags))
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
                                .nest(
                                    "/comments",
                                    Router::new()
                                        .route(
                                            "/",
                                            get(get_comments).post(post_comment.layer(
                                                middleware::from_fn_with_state(
                                                    state.clone(),
                                                    auth_middleware,
                                                ),
                                            )),
                                        )
                                        .route(
                                            "/:comment_id",
                                            patch(patch_comment).delete(delete_comment).layer(
                                                middleware::from_fn_with_state(
                                                    state.clone(),
                                                    auth_middleware,
                                                ),
                                            ),
                                        ),
                                )
                                .route(
                                    "/articles",
                                    get(get_list_articles)
                                        .put(put_list_article)
                                        .delete(delete_list_article)
                                        .layer(middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
                                        )),
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
                                .nest(
                                    "/comments",
                                    Router::new()
                                        .route(
                                            "/",
                                            get(get_comments).post(post_comment.layer(
                                                middleware::from_fn_with_state(
                                                    state.clone(),
                                                    auth_middleware,
                                                ),
                                            )),
                                        )
                                        .route(
                                            "/:comment_id",
                                            patch(patch_comment).delete(delete_comment).layer(
                                                middleware::from_fn_with_state(
                                                    state.clone(),
                                                    auth_middleware,
                                                ),
                                            ),
                                        ),
                                )
                                .route(
                                    "/articles",
                                    get(get_series_articles)
                                        .put(put_series_article)
                                        .patch(patch_series_article)
                                        .delete(delete_series_article)
                                        .layer(middleware::from_fn_with_state(
                                            state.clone(),
                                            auth_middleware,
                                        )),
                                ),
                        )
                        .layer(middleware::from_fn(resource_service_middleware)),
                ),
        )
        .layer(DefaultBodyLimit::max(1024 * 1024 * 50))
        .layer(prometheus_layer)
}
