mod config;
mod models;
mod repositories;
mod routes;
mod utils;

#[macro_use]
extern crate dotenv_codegen;

use axum::{
    http::StatusCode,
    routing::{get, patch, post, put},
    Router,
};
use axum_extra::extract::cookie::Key;
use axum_prometheus::PrometheusMetricLayer;
use config::{AppState, DATABASE_URL, PORT};
use dotenv::dotenv;
use repositories::PgRepository;
use routes::{
    article::{
        delete_article, delete_author, get_article, get_articles, get_articles_by_user,
        get_authors, patch_article, post_article, put_author,
    },
    comment::{delete_comment, get_comments, patch_comment, post_comment},
    healthchecker::health_checker,
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
use sqlx::postgres::PgPoolOptions;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

//TODO: add multithreading
//TODO: add rate limiter? mb middleware

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(["http://localhost:3000".parse().unwrap()]);

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(DATABASE_URL)
        .await
    {
        Ok(pool) => {
            info!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            error!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed migrations :(");

    let repository = PgRepository::new(&pool);

    let appstate = Arc::new(AppState {
        key: Key::generate(),
        repository,
    });

    //TODO: refactor to routes
    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/healthchecker", get(health_checker))
                .nest(
                    "/admin",
                    Router::new()
                        .route("/metrics", get(|| async move { metric_handle.render() }))
                        .nest(
                            "/tags",
                            Router::new().route("/", get(get_tags).post(post_tag)).nest(
                                "/:tag_id",
                                Router::new()
                                    .route("/", get(get_tag).patch(patch_tag).delete(delete_tag)),
                            ),
                        ),
                )
                .nest(
                    "/users",
                    Router::new()
                        .route("/", get(get_users).post(post_user))
                        .nest(
                            "/:user_id",
                            Router::new()
                                .route("/", get(get_user).patch(patch_user).delete(delete_user))
                                .route("/articles", get(get_articles_by_user))
                                .route("/lists", get(get_list_by_user))
                                .route("/series", get(get_series_by_user)),
                        ),
                )
                .nest(
                    "/articles",
                    Router::new()
                        .route("/", get(get_articles).post(post_article))
                        .nest(
                            "/:article_id",
                            Router::new()
                                .route("/", get(get_article))
                                .route("/authors", get(get_authors))
                                .route("/tags", get(get_tags))
                                .nest(
                                    "/comments",
                                    Router::new()
                                        .route("/", post(post_comment).get(get_comments))
                                        .route(
                                            "/:comment_id",
                                            patch(patch_comment).delete(delete_comment),
                                        ),
                                )
                                .nest(
                                    "/edit",
                                    Router::new()
                                        .route("/", patch(patch_article).delete(delete_article))
                                        .route(
                                            "/authors/:user_id",
                                            put(put_author).delete(delete_author),
                                        )
                                        .route(
                                            "/tags",
                                            put(StatusCode::NOT_IMPLEMENTED)
                                                .delete(StatusCode::NOT_IMPLEMENTED),
                                        ),
                                ),
                        ),
                )
                .nest(
                    "/lists",
                    Router::new()
                        .route("/", get(get_lists).post(post_list))
                        .nest(
                            "/:list_id",
                            Router::new()
                                .route("/", get(get_list).patch(patch_list).delete(delete_list))
                                .nest(
                                    "/articles",
                                    Router::new().route("/", get(get_list_articles)).route(
                                        "/:article_id",
                                        put(put_list_article).delete(delete_list_article),
                                    ),
                                ),
                        ),
                )
                .nest(
                    "/series",
                    Router::new()
                        .route("/", get(get_series).post(post_series))
                        .nest(
                            "/:series_id",
                            Router::new()
                                .route("/", patch(patch_series).delete(delete_series))
                                .nest(
                                    "/articles",
                                    Router::new().route("/", get(get_series_articles)).route(
                                        "/:article_id",
                                        put(put_series_article).delete(delete_series_article),
                                    ),
                                ),
                        ),
                ),
        )
        .with_state(appstate.clone())
        .layer(cors)
        .layer(prometheus_layer);

    let port = PORT.parse::<u16>().expect("PORT must be a number");

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TcpListener");

    axum::serve(listener, app).await.unwrap();
}
