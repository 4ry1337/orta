use std::sync::Arc;

use shared::{
    article::{
        article_service_server::ArticleService, AddAuthorRequest, CreateRequest, DeleteRequest,
        EditRequest, GetRequest, HistoryRequest, LikeRequest, PublishRequest, RemoveAuthorRequest,
        SearchRequest, SetTagsRequest, UnlikeRequest, UpdateRequest, VersionRequest,
    },
    common::{
        Article, ArticleVersion, ArticleVersions, FullArticle, FullArticles, MessageResponse, Tags,
    },
    models::{
        article_model::{AddAuthor, CreateArticle, DeleteAuthor, UpdateArticle},
        enums::TagStatus,
        tag_model::CreateTag,
    },
    repositories::{
        article_repository::{ArticleRepository, ArticleRepositoryImpl},
        tag_repository::{TagRepository, TagRepositoryImpl},
    },
};
use slug::slugify;
use tonic::{Request, Response, Status};
use tracing::{error, info};

use crate::{
    application::AppState,
    utils::{
        permissions::{is_owner, ContentType},
        split_cursor::parse_cursor,
    },
};

#[derive(Clone)]
pub struct ArticleServiceImpl {
    pub state: Arc<AppState>,
}

#[tonic::async_trait]
impl ArticleService for ArticleServiceImpl {
    async fn search(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<FullArticles>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get Articles Request {:?}", input);

        let mut id = None;
        let mut created_at = None;

        if let Some(cursor_str) = &input.cursor {
            (id, created_at) = match parse_cursor(cursor_str) {
                Ok(parsed) => parsed,
                Err(err) => {
                    error!("Parse error {}", err);
                    return Err(Status::invalid_argument("Invalid data"));
                }
            }
        };

        let articles = match ArticleRepositoryImpl::find_all(
            &mut transaction,
            input.query.as_deref(),
            input.limit,
            id,
            created_at,
            input.by_user.as_deref(),
            Some(true),
        )
        .await
        {
            Ok(articles) => articles,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let next_cursor = articles
            .iter()
            .nth(input.limit as usize - 1)
            .map(|item| format!("{}_{}", item.id, item.created_at.to_rfc3339()));

        let articles = articles
            .iter()
            .map(|article| FullArticle::from(article))
            .collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(FullArticles {
                articles,
                next_cursor,
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<FullArticle>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get Article Request {:?}", input);
        // TODO: check for user and public
        let article = match ArticleRepositoryImpl::find(
            &mut transaction,
            &input.article_id,
            input.by_user.as_deref(),
        )
        .await
        {
            Ok(article) => article,
            Err(err) => {
                error!("{:?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(FullArticle::from(&article))),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn create(&self, request: Request<CreateRequest>) -> Result<Response<Article>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Create Article Request {:?}", input);

        let article = match ArticleRepositoryImpl::create(
            &mut transaction,
            &CreateArticle {
                user_id: input.user_id.to_owned(),
                title: input.title.to_owned(),
                description: input.description.to_owned(),
            },
        )
        .await
        {
            Ok(article) => article,
            Err(err) => {
                error!("{:?}", err);
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
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn update(&self, request: Request<UpdateRequest>) -> Result<Response<Article>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Update Article Request {:?}", input);

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
                error!("{:?}", err);
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
                description: input.description.to_owned(),
            },
        )
        .await
        {
            Ok(article) => article,
            Err(err) => {
                error!("{:?}", err);
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
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Delete Article Request {:?}", input);

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
                error!("{:?}", err);

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
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(MessageResponse {
                message: format!("Deleted article: {}", article.id),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn add_author(
        &self,
        request: Request<AddAuthorRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Add Author Request {:?}", input);

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
                error!("{:?}", err);

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
                error!("{:?}", err);
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
            Ok(_) => Ok(Response::new(MessageResponse {
                message: format!("Author {} added to {}", authors.1, authors.0),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn remove_author(
        &self,
        request: Request<RemoveAuthorRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Remove Author Request {:?}", input);

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
                error!("{:?}", err);

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
                error!("{:?}", err);
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
            Ok(_) => Ok(Response::new(MessageResponse {
                message: format!("Author {} deleted from {}", authors.1, authors.0),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn like(
        &self,
        request: Request<LikeRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Like Article Request {:?}", input);

        let like =
            match ArticleRepositoryImpl::like(&mut transaction, &input.article_id, &input.user_id)
                .await
            {
                Ok(like) => like,
                Err(err) => {
                    error!("{:?}", err);
                    return Err(Status::internal("Something went wrong"));
                }
            };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(MessageResponse {
                message: format!("{} liked article {}", like.0, like.1),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn unlike(
        &self,
        request: Request<UnlikeRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Like Article Request {:?}", input);

        let like = match ArticleRepositoryImpl::unlike(
            &mut transaction,
            &input.article_id,
            &input.user_id,
        )
        .await
        {
            Ok(like) => like,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(MessageResponse {
                message: format!("{} unliked article {}", like.0, like.1),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Like Article Request {:?}", input);

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
                error!("{:?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::unknown("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };
        let article =
            match ArticleRepositoryImpl::publish(&mut transaction, &input.article_id).await {
                Ok(article) => article,
                Err(err) => {
                    error!("{:?}", err);
                    return Err(Status::internal("Something went wrong"));
                }
            };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(MessageResponse {
                message: format!("Published article: {}", article.id),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn history(
        &self,
        request: Request<HistoryRequest>,
    ) -> Result<Response<ArticleVersions>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get History Request {:?}", input);

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
                error!("{:?}", err);

                // TODO check if works
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let mut id = None;
        let mut created_at = None;

        if let Some(cursor_str) = &input.cursor {
            (id, created_at) = match parse_cursor(cursor_str) {
                Ok(parsed) => parsed,
                Err(err) => {
                    error!("Parse error {}", err);
                    return Err(Status::invalid_argument("Invalid data"));
                }
            }
        };

        let article_versions = match ArticleRepositoryImpl::history(
            &mut transaction,
            &input.article_id,
            input.limit,
            id,
            created_at,
        )
        .await
        {
            Ok(article_versions) => article_versions,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let next_cursor = article_versions
            .iter()
            .nth(input.limit as usize - 1)
            .map(|item| format!("{}_{}", item.id, item.created_at.to_rfc3339()));

        let article_versions = article_versions
            .iter()
            .map(|article_version| ArticleVersion::from(article_version))
            .collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(ArticleVersions {
                article_versions,
                next_cursor,
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn edit(
        &self,
        request: Request<EditRequest>,
    ) -> Result<Response<ArticleVersion>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get History Request {:?}", input);

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
                    error!("Forbidden");
                    return Err(Status::permission_denied("Forbidden"));
                }
            }
            Err(err) => {
                error!("{:?}", err);

                // TODO check if works
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let article_version = match ArticleRepositoryImpl::edit(
            &mut transaction,
            &input.article_id,
            &input.content,
            input.device_id.as_deref(),
        )
        .await
        {
            Ok(article_version) => article_version,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(ArticleVersion::from(&article_version))),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn version(
        &self,
        request: Request<VersionRequest>,
    ) -> Result<Response<ArticleVersion>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get Article Version Request {:?}", input);

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
                    error!("Forbidden");
                    return Err(Status::permission_denied("Forbidden"));
                }
            }
            Err(err) => {
                error!("{:?}", err);

                // TODO check if works
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let article_version =
            match ArticleRepositoryImpl::version(&mut transaction, &input.article_id).await {
                Ok(article_version) => article_version,
                Err(err) => {
                    error!("{:?}", err);
                    return Err(Status::internal("Something went wrong"));
                }
            };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(ArticleVersion::from(&article_version))),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn set_tags(
        &self,
        request: Request<SetTagsRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Set Article Tags Request {:?}", input);

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
                    error!("Forbidden");
                    return Err(Status::permission_denied("Forbidden"));
                }
            }
            Err(err) => {
                error!("{:?}", err);

                // TODO check if works
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        }

        match TagRepositoryImpl::remove_article_tags(&mut transaction, &input.article_id).await {
            Ok(res) => res,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        for tag in &input.tags {
            let tag_slug = match TagRepositoryImpl::create(
                &mut transaction,
                &CreateTag {
                    label: tag.to_owned(),
                    tag_status: TagStatus::Approved,
                },
            )
            .await
            {
                Ok(tag) => tag.slug,
                Err(err) => {
                    error!("{:?}", err);
                    return Err(Status::internal("Something went wrong"));
                }
            };

            match TagRepositoryImpl::add_article_tags(
                &mut transaction,
                &input.article_id,
                &tag_slug,
            )
            .await
            {
                Ok(_) => (),
                Err(err) => {
                    error!("{:?}", err);
                    return Err(Status::internal("Something went wrong"));
                }
            };
        }

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(MessageResponse {
                message: format!("Set Tags Article",),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}
