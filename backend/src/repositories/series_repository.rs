use axum::async_trait;
use sqlx::{Error, PgPool};

use crate::models::{
    article_model::Article,
    series_model::{CreateSeries, Series, UpdateSeries},
};

#[async_trait]
pub trait SeriesRepository<E> {
    async fn find_all(&self) -> Result<Vec<Series>, E>;
    async fn find_by_user(&self, user_id: i32) -> Result<Vec<Series>, E>;
    async fn create(&self, create_series: &CreateSeries) -> Result<Series, E>;
    async fn update(&self, update_series: &UpdateSeries) -> Result<Series, E>;
    async fn delete(&self, series_id: i32) -> Result<(), E>;
    async fn find_articles(&self, series_id: i32) -> Result<Vec<Article>, E>;
    async fn add_article(&self, series_id: i32, article_id: i32) -> Result<(), E>;
    async fn reorder_article(
        &self,
        series_id: i32,
        article_id: i32,
        new_order: f32,
    ) -> Result<(), E>;
    async fn remove_article(&self, series_id: i32, article_id: i32) -> Result<(), E>;
}

#[derive(Debug, Clone)]
pub struct PgSeriesRepository {
    db: PgPool,
}

impl PgSeriesRepository {
    pub fn new(db: PgPool) -> PgSeriesRepository {
        Self { db }
    }
}

#[async_trait]
impl SeriesRepository<Error> for PgSeriesRepository {
    async fn find_all(&self) -> Result<Vec<Series>, Error> {
        sqlx::query_as!(
            Series,
            r#"
            SELECT *
            FROM series
            GROUP BY id, user_id
            ORDER BY created_at DESC
            "#n
        )
        .fetch_all(&self.db)
        .await
    }

    async fn find_by_user(&self, user_id: i32) -> Result<Vec<Series>, Error> {
        sqlx::query_as!(
            Series,
            r#"
            SELECT *
            FROM series
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#n,
            user_id
        )
        .fetch_all(&self.db)
        .await
    }

    async fn create(&self, create_series: &CreateSeries) -> Result<Series, Error> {
        sqlx::query_as!(
            Series,
            r#"
            INSERT INTO series (
                user_id,
                label,
                image
            )
            VALUES ($1, $2, $3)
            RETURNING *
            "#n,
            create_series.user_id,
            create_series.label,
            create_series.image
        )
        .fetch_one(&self.db)
        .await
    }

    async fn update(&self, update_series: &UpdateSeries) -> Result<Series, Error> {
        sqlx::query_as!(
            Series,
            r#"
            UPDATE series
            SET 
                label = coalesce($2, series.label),
                image = coalesce($3, series.image)
            WHERE id = $1
            RETURNING *
            "#n,
            update_series.id,
            update_series.label,
            update_series.image
        )
        .fetch_one(&self.db)
        .await
    }

    async fn delete(&self, series_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            DELETE FROM series
            WHERE id = $1
            "#n,
            series_id
        )
        .execute(&self.db)
        .await;
        Ok(())
    }

    async fn find_articles(&self, series_id: i32) -> Result<Vec<Article>, Error> {
        sqlx::query_as!(
            Article,
            r#"
            SELECT a.*
            FROM articles a
            JOIN seriesarticle sa ON a.id = sa.article_id
            WHERE sa.series_id = $1
            ORDER BY sa."order" ASC
            "#n,
            series_id,
        )
        .fetch_all(&self.db)
        .await
    }

    //TODO: test following
    async fn add_article(&self, series_id: i32, article_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            INSERT INTO seriesarticle (series_id, article_id, "order")
            VALUES ($1, $2, (
                SELECT max("order")
                FROM seriesarticle
                WHERE series_id = $1
            ) + 100)
            "#n,
            series_id,
            article_id,
        )
        .execute(&self.db)
        .await;
        Ok(())
    }

    async fn reorder_article(
        &self,
        series_id: i32,
        article_id: i32,
        new_order: f32,
    ) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            UPDATE seriesarticle
            SET "order" = $3
            WHERE series_id = $1 AND article_id = $2
            "#n,
            series_id,
            article_id,
            new_order
        )
        .execute(&self.db)
        .await;
        Ok(())
    }

    async fn remove_article(&self, series_id: i32, article_id: i32) -> Result<(), Error> {
        let _ = sqlx::query!(
            r#"
            DELETE FROM seriesarticle
            WHERE series_id = $1 AND article_id = $2
            "#n,
            series_id,
            article_id
        )
        .execute(&self.db)
        .await;
        Ok(())
    }
}
