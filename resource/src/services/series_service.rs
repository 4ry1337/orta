use std::sync::Arc;

use shared::{
    models::series_model::{CreateSeries, UpdateSeries},
    repositories::series_repository::{SeriesRepository, SeriesRepositoryImpl},
    resource_proto::{
        series_service_server::SeriesService, AddArticleSeriesRequest, AddArticleSeriesResponse,
        CreateSeriesRequest, DeleteSeriesRequest, DeleteSeriesResponse, GetSeriesRequest,
        GetSeriesesRequest, GetSeriesesResponse, RemoveArticleSeriesRequest,
        RemoveArticleSeriesResponse, Series, UpdateSeriesRequest, UpdateSeriesResponse,
    },
    utils::params::Filter,
};
use tonic::{Request, Response, Status};
use tracing::error;

use crate::{
    application::AppState,
    utils::permissions::{is_owner, ContentType},
};

#[derive(Clone)]
pub struct SeriesServiceImpl {
    pub state: Arc<AppState>,
}

#[tonic::async_trait]
impl SeriesService for SeriesServiceImpl {
    async fn get_serieses(
        &self,
        request: Request<GetSeriesesRequest>,
    ) -> Result<Response<GetSeriesesResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let total = match SeriesRepositoryImpl::total(&mut transaction).await {
            Ok(total) => match total {
                Some(total) => total,
                None => 0,
            },
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let serieses =
            match SeriesRepositoryImpl::find_all(&mut transaction, &Filter::from(&input.params))
                .await
            {
                Ok(serieses) => serieses,
                Err(err) => {
                    error!("{:#?}", err);
                    return Err(Status::internal("Something went wrong"));
                }
            };

        let serieses = serieses.iter().map(|series| Series::from(series)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(GetSeriesesResponse {
                total,
                series: serieses,
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn get_series(
        &self,
        request: Request<GetSeriesRequest>,
    ) -> Result<Response<Series>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let series =
            match SeriesRepositoryImpl::find_by_slug(&mut transaction, &input.series_slug).await {
                Ok(list) => list,
                Err(err) => {
                    error!("{:#?}", err);
                    if let sqlx::error::Error::RowNotFound = err {
                        return Err(Status::not_found("List not found"));
                    }
                    return Err(Status::internal("Something went wrong"));
                }
            };

        // let articles = match SeriesRepositoryImpl::find_articles(&mut transaction, series.id).await
        // {
        //     Ok(articles) => articles,
        //     Err(err) => {
        //         error!("{:#?}", err);
        //         return Err(Status::internal("Something went wrong"));
        //     }
        // };
        //
        // let articles = articles
        //     .iter()
        //     .map(|article| FullArticle::from(article))
        //     .collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(Series::from(&series))),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn create_series(
        &self,
        request: Request<CreateSeriesRequest>,
    ) -> Result<Response<Series>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let series = match SeriesRepositoryImpl::create(
            &mut transaction,
            &CreateSeries {
                user_id: input.user_id,
                label: input.label.clone(),
                image: input.image.clone(),
            },
        )
        .await
        {
            Ok(series) => series,
            Err(err) => {
                error!("{:#?}", err);
                if let Some(database_error) = err.as_database_error() {
                    if let Some(constraint) = database_error.constraint() {
                        if constraint == "lists_user_id_fkey" {
                            return Err(Status::not_found("User not found"));
                        }
                        if constraint == "lists_slug_key" {
                            return Err(Status::internal("Retry"));
                        }
                    }
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(Series::from(&series))),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn update_series(
        &self,
        request: Request<UpdateSeriesRequest>,
    ) -> Result<Response<UpdateSeriesResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        match is_owner(
            &mut transaction,
            ContentType::Series,
            input.user_id,
            input.series_id,
        )
        .await
        {
            Ok(is_owner) => {
                if !is_owner {
                    return Err(Status::permission_denied("Forbidden"));
                }
            }
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::unknown("Series not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let series = match SeriesRepositoryImpl::update(
            &mut transaction,
            &UpdateSeries {
                id: input.series_id,
                label: input.label.clone(),
                image: input.image.clone(),
            },
        )
        .await
        {
            Ok(series) => series,
            Err(err) => {
                error!("{:#?}", err);
                if let Some(database_error) = err.as_database_error() {
                    if let Some(constraint) = database_error.constraint() {
                        if constraint == "series_slug_key" {
                            return Err(Status::internal("Retry"));
                        }
                    }
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(UpdateSeriesResponse {
                message: format!("Updated series: {}", series.label),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn delete_series(
        &self,
        request: Request<DeleteSeriesRequest>,
    ) -> Result<Response<DeleteSeriesResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        match is_owner(
            &mut transaction,
            ContentType::Series,
            input.user_id,
            input.series_id,
        )
        .await
        {
            Ok(is_owner) => {
                if !is_owner {
                    return Err(Status::permission_denied("Forbidden"));
                }
            }
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::unknown("Series not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let series = match SeriesRepositoryImpl::delete(&mut transaction, input.series_id).await {
            Ok(series) => series,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(DeleteSeriesResponse {
                message: format!("Deleted series: {}", series.id),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn add_article(
        &self,
        request: Request<AddArticleSeriesRequest>,
    ) -> Result<Response<AddArticleSeriesResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        match is_owner(
            &mut transaction,
            ContentType::Series,
            input.user_id,
            input.series_id,
        )
        .await
        {
            Ok(is_owner) => {
                if !is_owner {
                    return Err(Status::permission_denied("Forbidden"));
                }
            }
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::unknown("Series not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match is_owner(
            &mut transaction,
            ContentType::Article,
            input.user_id,
            input.article_id,
        )
        .await
        {
            Ok(is_owner) => {
                if !is_owner {
                    return Err(Status::permission_denied("Forbidden"));
                }
            }
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::unknown("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let reponse = match SeriesRepositoryImpl::add_article(
            &mut transaction,
            input.series_id,
            input.article_id,
        )
        .await
        {
            Ok(reponse) => reponse,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(()) => Ok(Response::new(AddArticleSeriesResponse {
                message: format!("Article {} added to Series {}", reponse.1, reponse.0),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn remove_article(
        &self,
        request: Request<RemoveArticleSeriesRequest>,
    ) -> Result<Response<RemoveArticleSeriesResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        match is_owner(
            &mut transaction,
            ContentType::Series,
            input.user_id,
            input.series_id,
        )
        .await
        {
            Ok(is_owner) => {
                if !is_owner {
                    return Err(Status::permission_denied("Forbidden"));
                }
            }
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::unknown("Series not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match is_owner(
            &mut transaction,
            ContentType::Article,
            input.user_id,
            input.article_id,
        )
        .await
        {
            Ok(is_owner) => {
                if !is_owner {
                    return Err(Status::permission_denied("Forbidden"));
                }
            }
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::unknown("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let reponse = match SeriesRepositoryImpl::remove_article(
            &mut transaction,
            input.series_id,
            input.article_id,
        )
        .await
        {
            Ok(reponse) => reponse,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(()) => Ok(Response::new(RemoveArticleSeriesResponse {
                message: format!("Article {} removed to Series {}", reponse.1, reponse.0),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}
