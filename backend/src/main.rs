mod config;
mod models;
mod repository;
mod routes;
mod utils;

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use anyhow::Context;
use axum::{
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use axum_core::{extract::FromRef, response::IntoResponse};
use axum_extra::extract::cookie::Key;
use axum_prometheus::PrometheusMetricLayer;
use config::Config;
use dotenv::dotenv;
use repository::repository::PgRepository;
use routes::{
    article::{create_article, delete_article, get_article, update_article},
    user::{create_user, delete_user, get_user, update_user},
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

    let repository = PgRepository::set(pool);

    let appstate = Arc::new(AppState {
        key: Key::generate(),
        config,
        repository,
    });

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                // .nest(
                //     "/auth",
                //     Router::new()
                //         .route("/signup", post(signup))
                //         .route("/signin", post(signin))
                //         .nest(
                //             "/session",
                //             Router::new()
                //                 .route(
                //                     "/",
                //                     post(StatusCode::NOT_IMPLEMENTED)
                //                         .patch(StatusCode::NOT_IMPLEMENTED),
                //                 )
                //                 .route(
                //                     "/:session_token",
                //                     get(StatusCode::NOT_IMPLEMENTED)
                //                         .delete(StatusCode::NOT_IMPLEMENTED),
                //                 ),
                //         )
                //         .nest(
                //             "/user",
                //             Router::new()
                //                 .route(
                //                     "/:idoremail",
                //                     get(StatusCode::NOT_IMPLEMENTED)
                //                         .delete(StatusCode::NOT_IMPLEMENTED)
                //                         .patch(StatusCode::NOT_IMPLEMENTED),
                //                 )
                //                 .nest(
                //                     "/account",
                //                     Router::new()
                //                         .route(
                //                             "/:provider/:provider_account_id",
                //                             delete(StatusCode::NOT_IMPLEMENTED)
                //                                 .get(StatusCode::NOT_IMPLEMENTED),
                //                         )
                //                         .route("/", post(StatusCode::NOT_IMPLEMENTED)),
                //                 ),
                //         ),
                // )
                .route("/healthchecker", get(health_checker_handler))
                .route("/metrics", get(|| async move { metric_handle.render() }))
                .nest(
                    "/user",
                    Router::new().route("/", post(create_user)).route(
                        "/:user_id",
                        get(get_user).patch(update_user).delete(delete_user),
                    ),
                )
                .route("/users", get(StatusCode::NOT_IMPLEMENTED))
                .nest(
                    "/aritlce",
                    Router::new().route("/", post(create_article)).route(
                        "/:article_id",
                        get(get_article)
                            .patch(update_article)
                            .delete(delete_article),
                    ),
                )
                .route("/articles", get(StatusCode::NOT_IMPLEMENTED)),
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

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Orta is running";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
