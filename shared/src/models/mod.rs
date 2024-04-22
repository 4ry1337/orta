pub mod account_model;
pub mod article_model;
pub mod comment_model;
pub mod enums;
pub mod list_model;
pub mod series_model;
pub mod tag_model;
pub mod user_model;

pub mod prelude {
    pub use crate::models::account_model::*;
    pub use crate::models::article_model::*;
    pub use crate::models::comment_model::*;
    pub use crate::models::enums::*;
    pub use crate::models::list_model::*;
    pub use crate::models::series_model::*;
    pub use crate::models::tag_model::*;
    pub use crate::models::user_model::*;
}
