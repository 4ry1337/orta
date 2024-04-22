pub mod configuration;
pub mod models;
pub mod repositories;
pub mod utils;
pub mod authproto {
    tonic::include_proto!("auth");
}
