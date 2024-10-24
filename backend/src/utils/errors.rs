use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    //#[error("Failed to acquire a Postgres connection from the pool")]
    //PoolError(#[source] sqlx::Error),
    //#[error(transparent)]
    //Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
