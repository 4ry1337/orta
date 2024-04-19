use axum::async_trait;
use sqlx::{Database, Error, Postgres, Transaction};

use crate::models::{
    article_model::Article,
    series_model::{CreateSeries, Series, UpdateSeries},
};

#[async_trait]
pub trait SeriesRepository<DB, E>
where
    DB: Database,
{
    async fn find_all(transaction: &mut Transaction<'_, DB>) -> Result<Vec<Series>, E>;
    async fn find_by_user(
        transaction: &mut Transaction<'_, DB>,
        user_id: i32,
    ) -> Result<Vec<Series>, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        create_series: &CreateSeries,
    ) -> Result<Series, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_series: &UpdateSeries,
    ) -> Result<Series, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, series_id: i32) -> Result<Series, E>;
    //TODO: get one series
    async fn find_articles(
        transaction: &mut Transaction<'_, DB>,
        series_id: i32,
    ) -> Result<Vec<Article>, E>;
    async fn add_article(
        transaction: &mut Transaction<'_, DB>,
        series_id: i32,
        article_id: i32,
    ) -> Result<(i32, i32), E>;
    async fn reorder_article(
        transaction: &mut Transaction<'_, DB>,
        series_id: i32,
        article_id: i32,
        new_order: f32,
    ) -> Result<(i32, i32), E>;
    async fn remove_article(
        transaction: &mut Transaction<'_, DB>,
        series_id: i32,
        article_id: i32,
    ) -> Result<(i32, i32), E>;
}

#[derive(Debug, Clone)]
pub struct SeriesRepositoryImpl;

#[async_trait]
impl SeriesRepository<Postgres, Error> for SeriesRepositoryImpl {
    async fn find_all(transaction: &mut Transaction<'_, Postgres>) -> Result<Vec<Series>, Error> {
        sqlx::query_as!(
            Series,
            r#"
            SELECT *
            FROM series
            GROUP BY id, user_id
            ORDER BY created_at DESC
            "#n
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find_by_user(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: i32,
    ) -> Result<Vec<Series>, Error> {
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
        .fetch_all(&mut **transaction)
        .await
    }

    async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        create_series: &CreateSeries,
    ) -> Result<Series, Error> {
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
        .fetch_one(&mut **transaction)
        .await
    }

    async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        update_series: &UpdateSeries,
    ) -> Result<Series, Error> {
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
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: i32,
    ) -> Result<Series, Error> {
        sqlx::query_as!(
            Series,
            r#"
            DELETE FROM series
            WHERE id = $1
            RETURNING *
            "#n,
            series_id
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn find_articles(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: i32,
    ) -> Result<Vec<Article>, Error> {
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
        .fetch_all(&mut **transaction)
        .await
    }

    //TODO: test following
    async fn add_article(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: i32,
        article_id: i32,
    ) -> Result<(i32, i32), Error> {
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
        .execute(&mut **transaction)
        .await;
        Ok((series_id, article_id))
    }

    async fn reorder_article(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: i32,
        article_id: i32,
        new_order: f32,
    ) -> Result<(i32, i32), Error> {
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
        .execute(&mut **transaction)
        .await;
        Ok((series_id, article_id))
    }

    async fn remove_article(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: i32,
        article_id: i32,
    ) -> Result<(i32, i32), Error> {
        let _ = sqlx::query!(
            r#"
            DELETE FROM seriesarticle
            WHERE series_id = $1 AND article_id = $2
            "#n,
            series_id,
            article_id
        )
        .execute(&mut **transaction)
        .await;
        Ok((series_id, article_id))
    }
}
