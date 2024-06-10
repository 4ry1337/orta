use std::sync::Arc;

use shared::{
    common::{FullArticle, FullArticles, List, Lists, MessageResponse},
    list::{
        list_service_server::ListService, AddArticleRequest, ArticlesRequest, CreateRequest,
        DeleteRequest, GetRequest, RemoveArticleRequest, SearchRequest, UpdateRequest,
    },
    models::{
        enums::Visibility,
        list_model::{CreateList, UpdateList},
    },
    repositories::{
        article_repository::{ArticleRepository, ArticleRepositoryImpl},
        list_repository::{ListRepository, ListRepositoryImpl},
    },
};
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
pub struct ListServiceImpl {
    pub state: Arc<AppState>,
}

#[tonic::async_trait]
impl ListService for ListServiceImpl {
    async fn search(&self, request: Request<SearchRequest>) -> Result<Response<Lists>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get Lists Request {:?}", input);

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

        let lists = match ListRepositoryImpl::find_all(
            &mut transaction,
            input.query.as_deref(),
            input.limit,
            id,
            created_at,
            input.by_user.as_deref(),
        )
        .await
        {
            Ok(lists) => lists,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let next_cursor = lists
            .iter()
            .nth(input.limit as usize - 1)
            .map(|item| format!("{}_{}", item.id, item.created_at.to_rfc3339()));

        let lists = lists.iter().map(|list| List::from(list)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(Lists { lists, next_cursor })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<List>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get List Request {:?}", input);

        let list = match ListRepositoryImpl::find(
            &mut transaction,
            &input.list_id,
            input.by_user.as_deref(),
        )
        .await
        {
            Ok(list) => list,
            Err(err) => {
                error!("{:?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("List not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(List::from(&list))),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn articles(
        &self,
        request: Request<ArticlesRequest>,
    ) -> Result<Response<FullArticles>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Get List Articles Request {:?}", input);

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

        let articles = match ListRepositoryImpl::find_articles(
            &mut transaction,
            input.limit,
            id,
            created_at,
            input.by_user.as_deref(),
            &input.list_id,
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

    async fn create(&self, request: Request<CreateRequest>) -> Result<Response<List>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Create List Request {:?}", input);

        let list = match ListRepositoryImpl::create(
            &mut transaction,
            &CreateList {
                user_id: input.user_id.to_owned(),
                label: input.label.to_owned(),
                image: input.image.to_owned(),
                visibility: Visibility::from(input.visibility()),
            },
        )
        .await
        {
            Ok(list) => list,
            Err(err) => {
                error!("{:?}", err);
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
            Ok(_) => Ok(Response::new(List::from(&list))),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn update(&self, request: Request<UpdateRequest>) -> Result<Response<List>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Update List Request {:?}", input);

        match is_owner(
            &mut transaction,
            ContentType::List,
            &input.user_id,
            &input.list_id,
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
                    return Err(Status::unknown("List not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let list = match ListRepositoryImpl::update(
            &mut transaction,
            &UpdateList {
                id: input.list_id.to_owned(),
                label: input.label.to_owned(),
                image: input.image.to_owned(),
                visibility: None,
            },
        )
        .await
        {
            Ok(list) => list,
            Err(err) => {
                error!("{:?}", err);
                if let Some(database_error) = err.as_database_error() {
                    if let Some(constraint) = database_error.constraint() {
                        if constraint == "lists_slug_key" {
                            return Err(Status::internal("Retry"));
                        }
                    }
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(List::from(&list))),
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

        info!("Delete List Request {:?}", input);

        match is_owner(
            &mut transaction,
            ContentType::List,
            &input.user_id,
            &input.list_id,
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
                    return Err(Status::unknown("List not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };
        let list = match ListRepositoryImpl::delete(&mut transaction, &input.list_id).await {
            Ok(list) => list,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(MessageResponse {
                message: format!("Deleted list: {}", list.id),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn add_article(
        &self,
        request: Request<AddArticleRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Add Article to List Request {:?}", input);

        match is_owner(
            &mut transaction,
            ContentType::List,
            &input.user_id,
            &input.list_id,
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
                    return Err(Status::unknown("List not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let article =
            match ArticleRepositoryImpl::find(&mut transaction, &input.article_id, None).await {
                Ok(article) => article,
                Err(err) => {
                    error!("{:?}", err);
                    if let sqlx::error::Error::RowNotFound = err {
                        return Err(Status::unknown("Article not found"));
                    }
                    return Err(Status::internal("Something went wrong"));
                }
            };

        let reponse =
            match ListRepositoryImpl::add_article(&mut transaction, &input.list_id, &article.id)
                .await
            {
                Ok(reponse) => reponse,
                Err(err) => {
                    error!("{:?}", err);
                    return Err(Status::internal("Something went wrong"));
                }
            };

        match transaction.commit().await {
            Ok(()) => Ok(Response::new(MessageResponse {
                message: format!("Article {} added to List {}", reponse.1, reponse.0),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn remove_article(
        &self,
        request: Request<RemoveArticleRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        info!("Remove Article from List Request {:?}", input);

        match is_owner(
            &mut transaction,
            ContentType::List,
            &input.user_id,
            &input.list_id,
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
                    return Err(Status::unknown("List not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let article =
            match ArticleRepositoryImpl::find(&mut transaction, &input.article_id, None).await {
                Ok(article) => article,
                Err(err) => {
                    error!("{:?}", err);
                    if let sqlx::error::Error::RowNotFound = err {
                        return Err(Status::unknown("Article not found"));
                    }
                    return Err(Status::internal("Something went wrong"));
                }
            };

        let reponse =
            match ListRepositoryImpl::remove_article(&mut transaction, &input.list_id, &article.id)
                .await
            {
                Ok(reponse) => reponse,
                Err(err) => {
                    error!("{:?}", err);
                    return Err(Status::internal("Something went wrong"));
                }
            };

        match transaction.commit().await {
            Ok(()) => Ok(Response::new(MessageResponse {
                message: format!("Article {} removed to List {}", reponse.1, reponse.0),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}
