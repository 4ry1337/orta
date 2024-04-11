mod config;
mod models;
mod repositories;
mod routes;
mod utils;

#[macro_use]
extern crate dotenv_codegen;

use axum::{routing::get, Router};
use axum_extra::extract::cookie::Key;
use axum_prometheus::PrometheusMetricLayer;
use config::{AppState, DATABASE_URL, PORT};
use dotenv::dotenv;
use repositories::PgRepository;
use routes::{admin::health_checker, article, comment, list, series, tags, user};
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
                .route("/admin/healthchecker", get(health_checker))
                .route(
                    "/admin/metrics",
                    get(|| async move { metric_handle.render() }),
                )
                .merge(user::router())
                .merge(article::router())
                .merge(list::router())
                .merge(series::router())
                .merge(comment::router())
                .merge(tags::router()),
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
