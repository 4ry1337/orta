use axum::routing::Router;

use axum_prometheus::PrometheusMetricLayer;

use crate::application::AppState;

// pub mod admin;
// pub mod article;
pub mod auth;
// pub mod comment;
// pub mod list;
// pub mod series;
// pub mod tags;
// pub mod user;

pub fn router(state: AppState) -> Router<AppState> {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    Router::new()
        .nest("/api", Router::new().merge(auth::router()))
        .layer(prometheus_layer)
}
// .nest(
//     "/users",
//     Router::new().route("/", get(get_users)).route(
//         "/:user_id",
//         get(get_user)
//             .patch(patch_user.layer(middleware::from_fn_with_state(
//                 state.clone(),
//                 auth_middleware,
//             )))
//             .delete(delete_user.layer(middleware::from_fn_with_state(
//                 state.clone(),
//                 auth_middleware,
//             ))),
//     ),
// ),
// .nest(
//     "/admin",
//     Router::new()
//         .nest("/users", Router::new().route("/", post(post_user)))
//         .nest(
//             "/tags",
//             Router::new()
//                 .route("/", get(get_tags).post(post_tag))
//                 .route(
//                     "/:tag_id",
//                     get(get_tag).patch(patch_tag).delete(delete_tag),
//                 ),
//         )
//         .route("/healthchecker", get(health_checker))
//         .route("/metrics", get(|| async move { metric_handle.render() }))
//         .layer(middleware::from_fn_with_state(
//             state.clone(),
//             auth_middleware,
//         )),
// )
// .nest(
//     "/articles",
//     Router::new()
//         .route(
//             "/",
//             get(get_articles).post(post_article.layer(
//                 middleware::from_fn_with_state(state.clone(), auth_middleware),
//             )),
//         )
//         .nest(
//             "/:article_id",
//             Router::new()
//                 .route("/", get(get_article))
//                 .route(
//                     "/comments",
//                     post(post_comment.layer(middleware::from_fn_with_state(
//                         state.clone(),
//                         auth_middleware,
//                     )))
//                     .get(get_comments),
//                 )
//                 .route(
//                     "/comments/:comment_id",
//                     patch(patch_comment).delete(delete_comment).layer(
//                         middleware::from_fn_with_state(
//                             state.clone(),
//                             auth_middleware,
//                         ),
//                     ),
//                 )
//                 .nest(
//                     "/edit",
//                     Router::new()
//                         .route("/", patch(patch_article).delete(delete_article))
//                         .route(
//                             "/authors/:user_id",
//                             put(put_author).delete(delete_author),
//                         )
//                         .route(
//                             "/tags",
//                             put(StatusCode::NOT_IMPLEMENTED)
//                                 .delete(StatusCode::NOT_IMPLEMENTED),
//                         )
//                         .layer(middleware::from_fn_with_state(
//                             state.clone(),
//                             auth_middleware,
//                         )),
//                 ),
//         ),
// ),
// .nest(
//     "/lists",
//     Router::new()
//         .route(
//             "/",
//             get(get_lists).post(post_list.layer(middleware::from_fn_with_state(
//                 state.clone(),
//                 auth_middleware,
//             ))),
//         )
//         .nest(
//             "/:list_id",
//             Router::new()
//                 .route(
//                     "/",
//                     get(get_list)
//                         .patch(patch_list.layer(middleware::from_fn_with_state(
//                             state.clone(),
//                             auth_middleware,
//                         )))
//                         .delete(delete_list.layer(middleware::from_fn_with_state(
//                             state.clone(),
//                             auth_middleware,
//                         ))),
//                 )
//                 .route("/articles", get(get_list_articles))
//                 .route(
//                     "/articles/:article_id",
//                     put(put_list_article).delete(delete_list_article).layer(
//                         middleware::from_fn_with_state(
//                             state.clone(),
//                             auth_middleware,
//                         ),
//                     ),
//                 ),
//         ),
// ),
// .nest(
//     "/series",
//     Router::new()
//         .route(
//             "/",
//             get(get_series).post(post_series.layer(
//                 middleware::from_fn_with_state(state.clone(), auth_middleware),
//             )),
//         )
//         .nest(
//             "/:series_id",
//             Router::new()
//                 .route(
//                     "/",
//                     get(get_series)
//                         .patch(patch_series.layer(middleware::from_fn_with_state(
//                             state.clone(),
//                             auth_middleware,
//                         )))
//                         .delete(delete_series.layer(
//                             middleware::from_fn_with_state(
//                                 state.clone(),
//                                 auth_middleware,
//                             ),
//                         )),
//                 )
//                 .route("/articles", get(get_series_articles))
//                 .route(
//                     "/articles/:article_id",
//                     put(put_series_article).delete(delete_series_article).layer(
//                         middleware::from_fn_with_state(
//                             state.clone(),
//                             auth_middleware,
//                         ),
//                     ),
//                 ),
//         ),
// ),
