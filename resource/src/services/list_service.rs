use std::sync::Arc;

use shared::{
    models::{
        enums::Visibility,
        list_model::{CreateList, UpdateList},
    },
    repositories::{
        article_repository::{ArticleRepository, ArticleRepositoryImpl},
        list_repository::{ListRepository, ListRepositoryImpl},
    },
    resource_proto::{
        list_service_server::ListService, AddArticleListRequest, AddArticleListResponse,
        CreateListRequest, DeleteListRequest, DeleteListResponse, FullArticle, GetListRequest,
        GetListResponse, GetListsRequest, GetListsResponse, List, RemoveArticleListRequest,
        RemoveArticleListResponse, UpdateListRequest, UpdateListResponse,
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
pub struct ListServiceImpl {
    pub state: Arc<AppState>,
}

#[tonic::async_trait]
impl ListService for ListServiceImpl {
    async fn get_lists(
        &self,
        request: Request<GetListsRequest>,
    ) -> Result<Response<GetListsResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let total = match ListRepositoryImpl::total(&mut transaction).await {
            Ok(total) => match total {
                Some(total) => total,
                None => 0,
            },
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let lists = match ListRepositoryImpl::find_all(
            &mut transaction,
            &Filter::from(&input.params),
        )
        .await
        {
            Ok(lists) => lists,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let lists = lists.iter().map(|list| List::from(list)).collect();

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(GetListsResponse { total, lists })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn get_list(
        &self,
        request: Request<GetListRequest>,
    ) -> Result<Response<GetListResponse>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let list = match ListRepositoryImpl::find_by_slug(&mut transaction, &input.list_slug).await
        {
            Ok(list) => list,
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::not_found("List not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let articles = match ListRepositoryImpl::find_articles(&mut transaction, list.id).await {
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
            Ok(_) => Ok(Response::new(GetListResponse {
                list: Some(List::from(&list)),
                articles,
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn create_list(
        &self,
        request: Request<CreateListRequest>,
    ) -> Result<Response<List>, Status> {
        let mut transaction = match self.state.db.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        let input = request.get_ref();

        let list = match ListRepositoryImpl::create(
            &mut transaction,
            &CreateList {
                user_id: input.user_id,
                label: input.label.clone(),
                image: input.image.clone(),
                visibility: Visibility::Public,
            },
        )
        .await
        {
            Ok(list) => list,
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
            Ok(_) => Ok(Response::new(List::from(&list))),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn update_list(
        &self,
        request: Request<UpdateListRequest>,
    ) -> Result<Response<UpdateListResponse>, Status> {
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
            ContentType::List,
            input.user_id,
            input.list_id,
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
                    return Err(Status::unknown("List not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let list = match ListRepositoryImpl::update(
            &mut transaction,
            &UpdateList {
                id: input.list_id,
                label: input.label.clone(),
                image: input.image.clone(),
                visibility: None,
            },
        )
        .await
        {
            Ok(list) => list,
            Err(err) => {
                error!("{:#?}", err);
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
            Ok(_) => Ok(Response::new(UpdateListResponse {
                message: format!("Updated list: {}", list.id),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn delete_list(
        &self,
        request: Request<DeleteListRequest>,
    ) -> Result<Response<DeleteListResponse>, Status> {
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
            ContentType::List,
            input.user_id,
            input.list_id,
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
                    return Err(Status::unknown("List not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };
        let list = match ListRepositoryImpl::delete(&mut transaction, input.list_id).await {
            Ok(list) => list,
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(Response::new(DeleteListResponse {
                message: format!("Deleted list: {}", list.id),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn add_article(
        &self,
        request: Request<AddArticleListRequest>,
    ) -> Result<Response<AddArticleListResponse>, Status> {
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
            ContentType::List,
            input.user_id,
            input.list_id,
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
                    return Err(Status::unknown("List not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let article = match ArticleRepositoryImpl::find(&mut transaction, input.article_id).await {
            Ok(article) => article,
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::unknown("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let reponse = match ListRepositoryImpl::add_article(
            &mut transaction,
            input.list_id,
            article.id,
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
            Ok(()) => Ok(Response::new(AddArticleListResponse {
                message: format!("Article {} added to List {}", reponse.1, reponse.0),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn remove_article(
        &self,
        request: Request<RemoveArticleListRequest>,
    ) -> Result<Response<RemoveArticleListResponse>, Status> {
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
            ContentType::List,
            input.user_id,
            input.list_id,
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
                    return Err(Status::unknown("List not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let article = match ArticleRepositoryImpl::find(&mut transaction, input.article_id).await {
            Ok(article) => article,
            Err(err) => {
                error!("{:#?}", err);
                if let sqlx::error::Error::RowNotFound = err {
                    return Err(Status::unknown("Article not found"));
                }
                return Err(Status::internal("Something went wrong"));
            }
        };

        let reponse =
            match ListRepositoryImpl::remove_article(&mut transaction, input.list_id, article.id)
                .await
            {
                Ok(reponse) => reponse,
                Err(err) => {
                    error!("{:#?}", err);
                    return Err(Status::internal("Something went wrong"));
                }
            };

        match transaction.commit().await {
            Ok(()) => Ok(Response::new(RemoveArticleListResponse {
                message: format!("Article {} removed to List {}", reponse.1, reponse.0),
            })),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}
