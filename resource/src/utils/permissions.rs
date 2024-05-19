use shared::repositories::{
    article_repository::{ArticleRepository, ArticleRepositoryImpl},
    comment_repository::{CommentRepository, CommentRepositoryImpl},
    list_repository::{ListRepository, ListRepositoryImpl},
    series_repository::{SeriesRepository, SeriesRepositoryImpl},
    user_repository::{UserRepository, UserRepositoryImpl},
};
use sqlx::{Postgres, Transaction};

#[derive(Clone)]
pub enum ContentType {
    User,
    Article,
    Comment,
    List,
    Series,
}

pub async fn is_owner(
    transaction: &mut Transaction<'_, Postgres>,
    content_type: ContentType,
    user_id: i32,
    target_id: i32,
) -> Result<bool, sqlx::Error> {
    match content_type {
        ContentType::Article => {
            let article = ArticleRepositoryImpl::find(transaction, target_id).await?;
            //TODO: write better or create new function in article_repository
            if article
                .users
                .is_some_and(|authors| authors.iter().any(|v| v.id == user_id))
            {
                return Ok(true);
            }
        }
        ContentType::Comment => {
            let comment = CommentRepositoryImpl::find(transaction, target_id).await?;
            if comment.commenter_id == user_id {
                return Ok(true);
            }
        }
        ContentType::List => {
            let list = ListRepositoryImpl::find(transaction, target_id).await?;
            if list.user_id == user_id {
                return Ok(true);
            }
        }
        ContentType::Series => {
            let series = SeriesRepositoryImpl::find(transaction, target_id).await?;
            if series.user_id == user_id {
                return Ok(true);
            }
        }
        ContentType::User => {
            let user = UserRepositoryImpl::find(transaction, target_id).await?;
            if user.id == user_id {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
