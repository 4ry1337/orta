mod config;
mod models;
mod repositories;
mod routes;
mod utils;

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use anyhow::Context;
use axum::{
    http::StatusCode,
    response::Response,
    routing::{get, patch, post},
    Router,
};
use axum_core::{extract::FromRef, response::IntoResponse};
use axum_extra::extract::cookie::Key;
use axum_prometheus::PrometheusMetricLayer;
use config::Config;
use dotenv::dotenv;
use repositories::PgRepository;
use routes::{
    article::{
        delete_article, delete_author, get_article, get_articles, get_articles_by_user,
        get_authors, patch_article, post_article, post_author,
    },
    comment::{delete_comment, get_comments, patch_comment, post_comment},
    list::{
        delete_list, delete_list_article, get_list_articles, get_list_by_user, get_lists,
        patch_list, post_list, post_list_article,
    },
    series::{
        delete_series, delete_series_article, get_series, get_series_articles, get_series_by_user,
        patch_series, post_series, post_series_article,
    },
    user::{delete_user, get_user, get_users, patch_user, post_user},
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

#[macro_use]
extern crate dotenv_codegen;

#[derive(Clone)]
pub struct AppState {
    pub key: Key,
    pub config: Config,
    pub repository: PgRepository,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

impl FromRef<AppState> for PgRepository {
    fn from_ref(state: &AppState) -> Self {
        state.repository.clone()
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let config = Config::init();

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(["http://localhost:3000".parse().unwrap()]);

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
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
        config,
        repository,
    });

    //TODO: refactor to routes
    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/healthchecker", get(health_checker_handler))
                .route("/metrics", get(|| async move { metric_handle.render() }))
                .nest(
                    "/admin",
                    Router::new().route("/tag", get(StatusCode::NOT_IMPLEMENTED)),
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
                                .route(
                                    "/",
                                    get(get_article).patch(patch_article).delete(delete_article),
                                )
                                .route(
                                    "/authors",
                                    get(get_authors).post(post_author).delete(delete_author),
                                )
                                .nest(
                                    "/comments",
                                    Router::new()
                                        .route("/", post(post_comment).get(get_comments))
                                        .route(
                                            "/:comment_id",
                                            patch(patch_comment).delete(delete_comment),
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
                                .route("/", patch(patch_list).delete(delete_list))
                                .nest(
                                    "/articles",
                                    Router::new().route("/", get(get_list_articles)).route(
                                        "/:article_id",
                                        post(post_list_article).delete(delete_list_article),
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
                                        post(post_series_article).delete(delete_series_article),
                                    ),
                                ),
                        ),
                ),
        )
        .with_state(appstate.clone())
        .layer(cors)
        .layer(prometheus_layer);

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, appstate.config.port));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("Failed to bind TcpListener")
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn health_checker_handler() -> Response {
    (StatusCode::OK, "Orta is running").into_response()
}
