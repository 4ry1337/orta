pub mod account_repository;
pub mod article_repository;
pub mod comment_repository;
pub mod list_repository;
pub mod series_repository;
pub mod tag_repository;
pub mod user_repository;

pub mod prelude {
    pub use crate::repositories::account_repository::{AccountRepository, AccountRepositoryImpl};
    pub use crate::repositories::article_repository::{ArticleRepository, ArticleRepositoryImpl};
    pub use crate::repositories::comment_repository::{CommentRepository, CommentRepositoryImpl};
    pub use crate::repositories::list_repository::{ListRepository, ListRepositoryImpl};
    pub use crate::repositories::series_repository::{SeriesRepository, SeriesRepositoryImpl};
    pub use crate::repositories::tag_repository::{TagRepository, TagRepositoryImpl};
    pub use crate::repositories::user_repository::{UserRepository, UserRepositoryImpl};
}
