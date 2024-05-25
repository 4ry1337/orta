pub mod configuration;
pub mod models;
pub mod repositories;
pub mod utils;
pub mod auth_proto {
    tonic::include_proto!("auth");
}
pub mod resource_proto {
    tonic::include_proto!("resource");
}
pub mod storage_proto {
    tonic::include_proto!("storage");
}
