pub mod configuration;
pub mod models;
pub mod repositories;
pub mod utils;
pub mod auth_proto {
    tonic::include_proto!("auth");
}
pub mod common {
    tonic::include_proto!("common");
}
pub mod user {
    tonic::include_proto!("user");
}
pub mod article {
    tonic::include_proto!("article");
}
pub mod list {
    tonic::include_proto!("list");
}
pub mod series {
    tonic::include_proto!("series");
}
pub mod comment {
    tonic::include_proto!("comment");
}
pub mod tag {
    tonic::include_proto!("tag");
}
pub mod storage_proto {
    tonic::include_proto!("storage");
}
