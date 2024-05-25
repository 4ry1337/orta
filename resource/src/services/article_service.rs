use std::sync::Arc;

use shared::{
    models::article_model::{AddAuthor, CreateArticle, DeleteAuthor, UpdateArticle},
    repositories::article_repository::{ArticleRepository, ArticleRepositoryImpl},
    resource_proto::{
        article_service_server::ArticleService, AddAuthorRequest, AddAuthorResponse, Article,
        ArticleVersion, CreateArticleRequest, DeleteArticleRequest, DeleteArticleResponse,
        FullArticle, GetArticleRequest, GetArticlesRequest, GetArticlesResponse, GetHistoryRequest,
        GetHistoryResponse, RemoveAuthorRequest, RemoveAuthorResponse, SaveArticleRequest,
        UpdateArticleRequest, UpdateTagsRequest, UpdateTagsResponse,
    },
    utils::params::Filter,
};
use tonic::{Request, Response, Status};
use tracing::{error, info};

use crate::{
    application::AppState,
    utils::permissions::{is_owner, ContentType},
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

        info!("Create Article Request {:#?}", input);

        let article = match ArticleRepositoryImpl::create(
            &mut transaction,
            &CreateArticle {
                user_id: input.user_id.to_owned(),
                title: input.title.to_owned(),
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

        info!("Get Articles Request {:#?}", input);

        let total = match ArticleRepositoryImpl::total(
            &mut transaction,
            input.usernames.to_owned(),
            input.list_id.as_deref(),
            input.series_id.as_deref(),
        )
        .await
        {
            Ok(total) => match total {
                Some(total) => total,
                None => 0,
            },
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let articles = match ArticleRepositoryImpl::find_all(
            &mut transaction,
            &Filter::from(&input.params),
            input.usernames.to_owned(),
            input.list_id.as_deref(),
            input.series_id.as_deref(),
        )
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

        info!("Get Article Request {:#?}", input);

        let article = match ArticleRepositoryImpl::find(&mut transaction, &input.article_id).await {
            Ok(article) => article,
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("Article not found"));
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
    ) -> Result<Response<Article>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Update Article Request {:#?}", input);

        match is_owner(
            &mut transaction,
            ContentType::Article,
            &input.user_id,
            &input.article_id,
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
                id: input.article_id.to_owned(),
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
            Ok(_) => Ok(Response::new(Article::from(&article))),
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

        info!("Delete Article Request {:#?}", input);

        match is_owner(
            &mut transaction,
            ContentType::Article,
            &input.user_id,
            &input.article_id,
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

        let article = match ArticleRepositoryImpl::delete(&mut transaction, &input.article_id).await
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

        info!("Add Author Request {:#?}", input);

        match is_owner(
            &mut transaction,
            ContentType::Article,
            &input.author_id,
            &input.article_id,
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
                user_id: input.user_id.to_owned(),
                article_id: input.article_id.to_owned(),
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

        info!("Remove Author Request {:#?}", input);

        match is_owner(
            &mut transaction,
            ContentType::Article,
            &input.author_id,
            &input.article_id,
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
                user_id: input.user_id.to_owned(),
                article_id: input.article_id.to_owned(),
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

    async fn get_history(
        &self,
        request: Request<GetHistoryRequest>,
    ) -> Result<Response<GetHistoryResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get History Request {:#?}", input);

        match is_owner(
            &mut transaction,
            ContentType::Article,
            &input.user_id,
            &input.article_id,
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

        let total = match ArticleRepositoryImpl::versions(&mut transaction, &input.article_id).await
        {
            Ok(total) => match total {
                Some(total) => total,
                None => 0,
            },
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let article_versions = match ArticleRepositoryImpl::history(
            &mut transaction,
            &input.article_id,
            &Filter::from(&input.params),
        )
        .await
        {
            Ok(article_versions) => article_versions,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let article_versions = article_versions
            .iter()
            .map(|article_version| ArticleVersion::from(article_version))
            .collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(GetHistoryResponse {
                total,
                article_versions,
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn save(
        &self,
        request: Request<SaveArticleRequest>,
    ) -> Result<Response<ArticleVersion>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get History Request {:#?}", input);

        match is_owner(
            &mut transaction,
            ContentType::Article,
            &input.user_id,
            &input.article_id,
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

        let article_version = match ArticleRepositoryImpl::save(
            &mut transaction,
            &input.article_id,
            &input.content,
            input.device_id.as_deref(),
        )
        .await
        {
            Ok(article_version) => article_version,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(ArticleVersion::from(&article_version))),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}
