use axum::async_trait;
use sqlx::{Error, PgPool};

use crate::models::comment_model::{Comment, CreateComment, UpdateComment};

#[async_trait()]
pub trait CommentRepository<E> {
    async fn find_by_article_id(&self, article_id: i32) -> Result<Vec<Comment>, E>;
    async fn create(&self, create_comment: &CreateComment) -> Result<Comment, E>;
    async fn update(&self, update_comment: &UpdateComment) -> Result<Comment, E>;
    async fn delete(&self, comment_id: i32) -> Result<(), E>;
}

#[derive(Debug, Clone)]
pub struct PgCommentRepository {
    db: PgPool,
}

impl PgCommentRepository {
    pub fn new(db: PgPool) -> PgCommentRepository {
        Self { db }
    }
}

#[async_trait]
impl CommentRepository<Error> for PgCommentRepository {
    async fn find_by_article_id(&self, article_id: i32) -> Result<Vec<Comment>, Error> {
        sqlx::query_as!(
            Comment,
            r#"
            SELECT *
            FROM comments
            WHERE article_id = $1
            "#n,
            article_id,
        )
        .fetch_all(&self.db)
        .await
    }

    async fn create(&self, create_comment: &CreateComment) -> Result<Comment, Error> {
        sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comments (
                commenter_id,
                content,
                article_id
            ) VALUES ($1, $2, $3)
            RETURNING *
            "#n,
            create_comment.user_id,
            create_comment.content,
            create_comment.article_id,
        )
        .fetch_one(&self.db)
        .await
    }

    async fn update(&self, update_comment: &UpdateComment) -> Result<Comment, Error> {
        sqlx::query_as!(
            Comment,
            r#"
            UPDATE comments
            SET content = COALESCE($3, comments.content)
            WHERE id = $1 AND commenter_id = $2
            RETURNING *
            "#n,
            update_comment.id,
            update_comment.user_id,
            update_comment.content
        )
        .fetch_one(&self.db)
        .await
    }

    async fn delete(&self, comment_id: i32) -> Result<(), Error> {
        let _ = sqlx::query_as!(
            Comment,
            r#"
            DELETE FROM articles
            WHERE id = $1
            "#n,
            comment_id,
        )
        .execute(&self.db)
        .await;
        Ok(())
    }
}
