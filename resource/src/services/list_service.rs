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
        CreateListRequest, DeleteListRequest, DeleteListResponse, GetListRequest, GetListsRequest,
        GetListsResponse, List, RemoveArticleListRequest, RemoveArticleListResponse,
        UpdateListRequest,
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
    async fn get_lists(
        &self,
        request: Request<GetListsRequest>,
    ) -> Result<Response<GetListsResponse>, Status> {
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
            input.user_id.as_deref(),
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
            Ok(_) => Ok(Response::new(GetListsResponse { lists, next_cursor })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }

    async fn get_list(&self, request: Request<GetListRequest>) -> Result<Response<List>, Status> {
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

    async fn create_list(
        &self,
        request: Request<CreateListRequest>,
    ) -> Result<Response<List>, Status> {
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
                visibility: Visibility::Public,
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

    async fn update_list(
        &self,
        request: Request<UpdateListRequest>,
    ) -> Result<Response<List>, Status> {
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

    async fn delete_list(
        &self,
        request: Request<DeleteListRequest>,
    ) -> Result<Response<DeleteListResponse>, Status> {
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
            Ok(_) => Ok(Response::new(DeleteListResponse {
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
        request: Request<AddArticleListRequest>,
    ) -> Result<Response<AddArticleListResponse>, Status> {
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
            Ok(()) => Ok(Response::new(AddArticleListResponse {
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
        request: Request<RemoveArticleListRequest>,
    ) -> Result<Response<RemoveArticleListResponse>, Status> {
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
            Ok(()) => Ok(Response::new(RemoveArticleListResponse {
                message: format!("Article {} removed to List {}", reponse.1, reponse.0),
            })),
            Err(err) => {
                error!("{:?}", err);
                return Err(Status::internal("Something went wrong"));
            }
        }
    }
}
