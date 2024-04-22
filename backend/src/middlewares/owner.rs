use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use shared::repositories::prelude::*;
use shared::utils::jwt::AccessTokenPayload;
use tracing::error;

use crate::application::AppState;

#[derive(Clone)]
pub enum ContentType {
    User,
    Article,
    Comment,
    List,
    Series,
}

#[derive(Clone)]
pub struct ContentOwnerState {
    pub r#type: ContentType,
    pub appstate: Arc<AppState>,
}
pub async fn content_owner_middleware(
    State(state): State<ContentOwnerState>,
    Extension(token): Extension<AccessTokenPayload>,
    Path(params): Path<HashMap<String, String>>,
    req: Request,
    next: Next,
) -> Response {
    let mut transaction = match state.appstate.db.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            error!("{:#?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
        }
    };
    match state.r#type {
        ContentType::Article => {
            let target_id = match params.get("article_id") {
                Some(target_id) => match target_id.parse::<i32>() {
                    Ok(target_id) => target_id,
                    Err(err) => {
                        error!("Error parsing params {:#?}", err);
                        return (StatusCode::BAD_REQUEST, "Wrong parameter").into_response();
                    }
                },
                None => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response()
                }
            };
            match ArticleRepositoryImpl::get_authors(&mut transaction, target_id).await {
                Ok(users) => {
                    //TODO: write better or create new function in article_repository
                    if users.iter().any(|v| v.id == token.user_id) {
                        match transaction.commit().await {
                            Ok(_) => return next.run(req).await,
                            Err(err) => {
                                error!("{:#?}", err);
                                return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                                    .into_response();
                            }
                        }
                    }
                    return (StatusCode::FORBIDDEN).into_response();
                }
                Err(err) => {
                    error!("{:#?}", err);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response();
                }
            }
        }
        ContentType::Comment => {
            let target_id = match params.get("comment_id") {
                Some(target_id) => match target_id.parse::<i32>() {
                    Ok(target_id) => target_id,
                    Err(err) => {
                        error!("Error parsing params {:#?}", err);
                        return (StatusCode::BAD_REQUEST, "Wrong parameter").into_response();
                    }
                },
                None => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response()
                }
            };
            match CommentRepositoryImpl::find(&mut transaction, target_id).await {
                Ok(comment) => {
                    if comment.commenter_id == token.user_id {
                        match transaction.commit().await {
                            Ok(_) => return next.run(req).await,
                            Err(err) => {
                                error!("{:#?}", err);
                                return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                                    .into_response();
                            }
                        }
                    }
                    return (StatusCode::FORBIDDEN).into_response();
                }
                Err(err) => {
                    error!("{:#?}", err);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response();
                }
            }
        }
        ContentType::List => {
            let target_id = match params.get("list_id") {
                Some(target_id) => match target_id.parse::<i32>() {
                    Ok(target_id) => target_id,
                    Err(err) => {
                        error!("Error parsing params {:#?}", err);
                        return (StatusCode::BAD_REQUEST, "Wrong parameter").into_response();
                    }
                },
                None => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response()
                }
            };
            match ListRepositoryImpl::find(&mut transaction, target_id).await {
                Ok(list) => {
                    if list.user_id == token.user_id {
                        match transaction.commit().await {
                            Ok(_) => return next.run(req).await,
                            Err(err) => {
                                error!("{:#?}", err);
                                return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                                    .into_response();
                            }
                        }
                    }
                    return (StatusCode::FORBIDDEN).into_response();
                }
                Err(err) => {
                    error!("{:#?}", err);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response();
                }
            }
        }
        ContentType::Series => {
            let target_id = match params.get("series_id") {
                Some(target_id) => match target_id.parse::<i32>() {
                    Ok(target_id) => target_id,
                    Err(err) => {
                        error!("Error parsing params {:#?}", err);
                        return (StatusCode::BAD_REQUEST, "Wrong parameter").into_response();
                    }
                },
                None => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response()
                }
            };
            match SeriesRepositoryImpl::find(&mut transaction, target_id).await {
                Ok(series) => {
                    if series.user_id == token.user_id {
                        match transaction.commit().await {
                            Ok(_) => return next.run(req).await,
                            Err(err) => {
                                error!("{:#?}", err);
                                return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                                    .into_response();
                            }
                        }
                    }
                    return (StatusCode::FORBIDDEN).into_response();
                }
                Err(err) => {
                    error!("{:#?}", err);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response();
                }
            }
        }
        ContentType::User => {
            let target_id = match params.get("user_id") {
                Some(target_id) => match target_id.parse::<i32>() {
                    Ok(target_id) => target_id,
                    Err(err) => {
                        error!("Error parsing params {:#?}", err);
                        return (StatusCode::BAD_REQUEST, "Wrong parameter").into_response();
                    }
                },
                None => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response()
                }
            };
            match UserRepositoryImpl::find(&mut transaction, target_id).await {
                Ok(v) => {
                    if v.id == token.user_id {
                        match transaction.commit().await {
                            Ok(_) => return next.run(req).await,
                            Err(err) => {
                                error!("{:#?}", err);
                                return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                                    .into_response();
                            }
                        }
                    }
                    return (StatusCode::FORBIDDEN).into_response();
                }
                Err(err) => {
                    error!("{:#?}", err);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
                        .into_response();
                }
            }
        }
    }
}
