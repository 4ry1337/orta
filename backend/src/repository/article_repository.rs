use axum::async_trait;
use sqlx::{Error, PgPool};

use crate::models::article_model::Article;

#[async_trait]
pub trait ArticleRepository<T, E> {
    fn set(db: T) -> Self;
    async fn create(&self, user_id: i32) -> Result<Article, E>;
    async fn get_by_id(&self, article_id: i32) -> Result<Article, E>;
    async fn get_by_users(&self, users: &[i32]) -> Result<Vec<Article>, E>;
    async fn delete(&self, article_id: i32) -> Result<(), E>;
    // async fn get_article_by_id(&self, article_id: Uuid) -> Result<Option<Article>, E>;
}

#[derive(Debug, Clone)]
pub struct PgArticleRepository {
    db: PgPool,
}

#[async_trait]
impl ArticleRepository<PgPool, Error> for PgArticleRepository {
    fn set(db: PgPool) -> PgArticleRepository {
        Self { db }
    }
    async fn create(&self, user_id: i32) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"insert into articles (user_ids) values(ARRAY[$1]::INTEGER[]) returning *"#n,
            user_id
        )
        .fetch_one(&self.db)
        .await
    }
    async fn get_by_id(&self, article_id: i32) -> Result<Article, Error> {
        sqlx::query_as!(
            Article,
            r#"select * from articles where id = $1"#n,
            article_id
        )
        .fetch_one(&self.db)
        .await
    }

    async fn get_by_users(&self, users: &[i32]) -> Result<Vec<Article>, Error> {
        sqlx::query_as!(
            Article,
            r#"select * from articles where user_ids @> $1"#n,
            &users
        )
        .fetch_all(&self.db)
        .await
    }
    async fn delete(&self, article_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(r#"delete from articles where id = $1"#n, article_id)
            .execute(&self.db)
            .await;
        Ok(())
    }
}
