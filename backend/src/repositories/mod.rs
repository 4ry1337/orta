use sqlx::PgPool;

use self::{
    article_repository::PgArticleRepository, comment_repository::PgCommentRepository,
    list_repository::PgListRepository, series_repository::PgSeriesRepository,
    tag_repository::PgTagRepository, user_repository::PgUserRepository,
};

pub mod article_repository;
pub mod comment_repository;
pub mod list_repository;
pub mod series_repository;
pub mod tag_repository;
pub mod user_repository;

#[derive(Debug, Clone)]
pub struct PgRepository {
    pub user: PgUserRepository,
    pub article: PgArticleRepository,
    pub comment: PgCommentRepository,
    pub list: PgListRepository,
    pub series: PgSeriesRepository,
    pub tag: PgTagRepository,
}

impl PgRepository {
    pub fn new(db: &PgPool) -> PgRepository {
        Self {
            user: PgUserRepository::new(db.clone()),
            article: PgArticleRepository::new(db.clone()),
            comment: PgCommentRepository::new(db.clone()),
            list: PgListRepository::new(db.clone()),
            tag: PgTagRepository::new(db.clone()),
            series: PgSeriesRepository::new(db.clone()),
        }
    }
}
