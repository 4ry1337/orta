use axum::extract::FromRef;
use sqlx::PgPool;

use self::{
    account_repository::PgAccountRepository, article_repository::PgArticleRepository,
    comment_repository::PgCommentRepository, list_repository::PgListRepository,
    series_repository::PgSeriesRepository, tag_repository::PgTagRepository,
    user_repository::PgUserRepository,
};

pub mod account_repository;
pub mod article_repository;
pub mod comment_repository;
pub mod list_repository;
pub mod password_repository;
pub mod series_repository;
pub mod tag_repository;
pub mod user_repository;

#[derive(Debug, Clone)]
pub struct PgRepository {
    pub account: PgAccountRepository,
    pub users: PgUserRepository,
    pub articles: PgArticleRepository,
    pub comments: PgCommentRepository,
    pub lists: PgListRepository,
    pub series: PgSeriesRepository,
    pub tags: PgTagRepository,
}

impl PgRepository {
    pub fn new(db: &PgPool) -> PgRepository {
        Self {
            account: PgAccountRepository::new(db.clone()),
            users: PgUserRepository::new(db.clone()),
            articles: PgArticleRepository::new(db.clone()),
            comments: PgCommentRepository::new(db.clone()),
            lists: PgListRepository::new(db.clone()),
            tags: PgTagRepository::new(db.clone()),
            series: PgSeriesRepository::new(db.clone()),
        }
    }
}
