use std::sync::Arc;

use shared::{
    models::article_model::{AddAuthor, CreateArticle, DeleteAuthor, UpdateArticle},
    repositories::article_repository::{ArticleRepository, ArticleRepositoryImpl},
    resource_proto::{
        article_service_server::ArticleService, AddAuthorRequest, AddAuthorResponse, Article,
        CreateArticleRequest, DeleteArticleRequest, DeleteArticleResponse, FullArticle,
        GetArticleRequest, GetArticlesRequest, GetArticlesResponse, RemoveAuthorRequest,
        RemoveAuthorResponse, UpdateArticleRequest, UpdateArticleResponse, UpdateTagsRequest,
        UpdateTagsResponse,
    },
    utils::params::Filter,
};
use tonic::{Request, Response, Status};
use tracing::error;

use crate::{
    application::AppState,
    permissions::{is_owner, ContentType},
};

#[derive(Clone)]
pub struct ArticleServiceImpl {
    pub state: Arc<AppState>,
}

#[tonic::async_trait]
impl ArticleService for ArticleServiceImpl {
    async fn create_article(
        &self,
        request: Request<CreateArticleRequest>,
    ) -> Result<Response<Article>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let article = match ArticleRepositoryImpl::create(
            &mut transaction,
            &CreateArticle {
                user_id: input.user_id,
                title: input.title.clone(),
            },
        )
        .await
        {
            Ok(article) => article,
            Err(err) => {
                error!("{:#?}", err);
                if let Some(database_error) = err.as_database_error() {
                    if let Some(constraint) = database_error.constraint() {
                        if constraint == "authors_author_id_fkey" {
                            return Err(Status::not_found("User not found"));
                        }
                        if constraint == "articles_slug_key" {
                            return Err(Status::internal("Retry"));
                        }
                    }
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(Article::from(&article))),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn get_articles(
        &self,
        request: Request<GetArticlesRequest>,
    ) -> Result<Response<GetArticlesResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let total = match ArticleRepositoryImpl::total(&mut transaction).await {
            Ok(total) => match total {
                Some(total) => total,
                None => 0,
            },
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let articles =
            match ArticleRepositoryImpl::find_all(&mut transaction, &Filter::from(&input.params))
                .await
            {
                Ok(articles) => articles,
                Err(err) => {
                    error!("{:#?}", err);
                    return Err(Status::internal("Something went wrong"));
                }
            };

        let articles = articles
            .iter()
            .map(|article| FullArticle::from(article))
            .collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(GetArticlesResponse { total, articles })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn get_article(
        &self,
        request: Request<GetArticleRequest>,
    ) -> Result<Response<FullArticle>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let article = match ArticleRepositoryImpl::find_by_slug(
            &mut transaction,
            &input.article_slug,
        )
        .await
        {
            Ok(article) => article,
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("User not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(FullArticle::from(&article))),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn update_article(
        &self,
        request: Request<UpdateArticleRequest>,
    ) -> Result<Response<UpdateArticleResponse>, Status> {
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

        let article = match ArticleRepositoryImpl::update(
            &mut transaction,
            &UpdateArticle {
                id: input.article_id,
                title: input.title.clone(),
            },
        )
        .await
        {
            Ok(article) => article,
            Err(err) => {
                error!("{:#?}", err);
                if let Some(database_error) = err.as_database_error() {
                    if let Some(constraint) = database_error.constraint() {
                        if constraint == "articles_slug_key" {
                            return Err(Status::internal("Retry"));
                        }
                    }
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(UpdateArticleResponse {
                message: format!("Updated article: {}", article.id),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn delete_article(
        &self,
        request: Request<DeleteArticleRequest>,
    ) -> Result<Response<DeleteArticleResponse>, Status> {
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

                // TODO check if works
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let article = match ArticleRepositoryImpl::delete(&mut transaction, input.article_id).await
        {
            Ok(article) => article,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(DeleteArticleResponse {
                message: format!("Deleted article: {}", article.id),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn add_author(
        &self,
        request: Request<AddAuthorRequest>,
    ) -> Result<Response<AddAuthorResponse>, Status> {
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

                // TODO check if works
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let authors = match ArticleRepositoryImpl::add_author(
            &mut transaction,
            &AddAuthor {
                user_id: input.user_id,
                article_id: input.article_id,
            },
        )
        .await
        {
            Ok(users) => users,
            Err(err) => {
                error!("{:#?}", err);
                if let Some(database_error) = err.as_database_error() {
                    if let Some(constraint) = database_error.constraint() {
                        if constraint == "authors_author_id_fkey" {
                            return Err(Status::not_found("User not found"));
                        }
                        if constraint == "authors_article_id_fkey" {
                            return Err(Status::not_found("Article not found"));
                        }
                    }
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(AddAuthorResponse {
                message: format!("Author {} added to {}", authors.1, authors.0),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn remove_author(
        &self,
        request: Request<RemoveAuthorRequest>,
    ) -> Result<Response<RemoveAuthorResponse>, Status> {
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

                // TODO check if works
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let authors = match ArticleRepositoryImpl::delete_author(
            &mut transaction,
            &DeleteAuthor {
                user_id: input.user_id,
                article_id: input.article_id,
            },
        )
        .await
        {
            Ok(users) => users,
            Err(err) => {
                error!("{:#?}", err);
                if let Some(database_error) = err.as_database_error() {
                    if let Some(constraint) = database_error.constraint() {
                        if constraint == "authors_author_id_fkey" {
                            return Err(Status::not_found("User not found"));
                        }
                        if constraint == "authors_article_id_fkey" {
                            return Err(Status::not_found("Article not found"));
                        }
                    }
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(RemoveAuthorResponse {
                message: format!("Author {} deleted from {}", authors.1, authors.0),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn update_tags(
        &self,
        request: Request<UpdateTagsRequest>,
    ) -> Result<Response<UpdateTagsResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }
}
