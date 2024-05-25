use async_trait::async_trait;
use sqlx::{Database, Error, Postgres, Transaction};

use crate::{
    models::series_model::{CreateSeries, Series, UpdateSeries},
    utils::params::Filter,
};

#[async_trait]
pub trait SeriesRepository<DB, E>
where
    DB: Database,
{
    async fn total(transaction: &mut Transaction<'_, DB>) -> Result<Option<i64>, E>;
    async fn find_all(
        transaction: &mut Transaction<'_, DB>,
        filters: &Filter,
    ) -> Result<Vec<Series>, E>;
    async fn create(
        transaction: &mut Transaction<'_, DB>,
        create_series: &CreateSeries,
    ) -> Result<Series, E>;
    async fn update(
        transaction: &mut Transaction<'_, DB>,
        update_series: &UpdateSeries,
    ) -> Result<Series, E>;
    async fn delete(transaction: &mut Transaction<'_, DB>, series_id: &str) -> Result<Series, E>;
    async fn find(transaction: &mut Transaction<'_, DB>, series_id: &str) -> Result<Series, E>;
    async fn add_article(
        transaction: &mut Transaction<'_, DB>,
        series_id: &str,
        article_id: &str,
    ) -> Result<(String, String), E>;
    async fn reorder_article(
        transaction: &mut Transaction<'_, DB>,
        series_id: &str,
        article_id: &str,
        new_order: f32,
    ) -> Result<(String, String), E>;
    async fn remove_article(
        transaction: &mut Transaction<'_, DB>,
        series_id: &str,
        article_id: &str,
    ) -> Result<(String, String), E>;
}

#[derive(Debug, Clone)]
pub struct SeriesRepositoryImpl;

#[async_trait]
impl SeriesRepository<Postgres, Error> for SeriesRepositoryImpl {
    async fn total(transaction: &mut Transaction<'_, Postgres>) -> Result<Option<i64>, Error> {
        sqlx::query_scalar!("SELECT COUNT(*) FROM series")
            .fetch_one(&mut **transaction)
            .await
    }

    async fn find_all(
        transaction: &mut Transaction<'_, Postgres>,
        filters: &Filter,
    ) -> Result<Vec<Series>, Error> {
        sqlx::query_as!(
            Series,
            r#"
            SELECT *
            FROM series
            ORDER BY $1
            LIMIT $2
            OFFSET $3
            "#n,
            filters.order_by,
            filters.limit,
            filters.offset
        )
        .fetch_all(&mut **transaction)
        .await
    }

    async fn find(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: &str,
    ) -> Result<Series, Error> {
        sqlx::query_as!(
            Series,
            r#"
            SELECT *
            FROM series
            WHERE id = $1
            "#n,
            series_id
        )
        .fetch_one(&mut **transaction)
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
            update_series.image,
        )
        .fetch_one(&mut **transaction)
        .await
    }

    async fn delete(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: &str,
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

    //TODO: test following
    async fn add_article(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: &str,
        article_id: &str,
    ) -> Result<(String, String), Error> {
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
        Ok((series_id.to_string(), article_id.to_string()))
    }

    async fn reorder_article(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: &str,
        article_id: &str,
        new_order: f32,
    ) -> Result<(String, String), Error> {
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
        Ok((series_id.to_string(), article_id.to_string()))
    }

    async fn remove_article(
        transaction: &mut Transaction<'_, Postgres>,
        series_id: &str,
        article_id: &str,
    ) -> Result<(String, String), Error> {
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
        Ok((series_id.to_string(), article_id.to_string()))
    }
}
