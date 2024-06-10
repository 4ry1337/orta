use axum::{
    extract::{Path, Query, Request, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Extension, Json,
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use serde::Deserialize;
use serde_json::json;
use shared::{
    models::{
        article_model::FullArticle,
        list_model::List,
        series_model::Series,
        user_model::{FullUser, User},
    },
    user::{
        user_service_client::UserServiceClient, ArticlesRequest, FeedRequest, FollowRequest,
        FollowersRequest, FollowingRequest, GetRequest, ListsRequest, SearchRequest,
        SeriesesRequest, UnfollowRequest, UpdateRequest,
    },
    utils::jwt::{AccessToken, AccessTokenPayload, JWT},
};
use tonic::transport::Channel;
use tracing::{error, info};

use crate::{
    application::AppState,
    utils::{
        mapper::code_to_statudecode,
        params::{CursorPagination, PathParams, ResultPaging},
    },
};

#[derive(Debug, Deserialize)]
pub struct UsersQueryParams {
    query: Option<String>,
}

pub async fn get_users(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    Query(query): Query<UsersQueryParams>,
    Query(cursor): Query<CursorPagination>,
    State(_state): State<AppState>,
) -> Response {
    info!("Get Users Request {:?} {:?}", query, cursor);
    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();
    match UserServiceClient::new(channel)
        .search(SearchRequest {
            query: query.query,
            limit: cursor.limit,
            cursor: cursor.cursor,
            by_user,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<FullUser> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res.users.iter().map(|user| FullUser::from(user)).collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn get_user(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();
    info!("Get User Request {:?}", params);
    let username = match params.username {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    match UserServiceClient::new(channel)
        .get(GetRequest { username, by_user })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(FullUser::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn get_user_articles(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    Query(cursor): Query<CursorPagination>,
    Path(params): Path<PathParams>,
) -> Response {
    info!("Get User Articles Request {:?}", params);

    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();

    let username = match params.username {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    match UserServiceClient::new(channel)
        .articles(ArticlesRequest {
            cursor: cursor.cursor,
            limit: cursor.limit,
            username,
            by_user,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<FullArticle> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res
                        .articles
                        .iter()
                        .map(|article| FullArticle::from(article))
                        .collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn get_user_drafts(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    Query(cursor): Query<CursorPagination>,
) -> Response {
    info!("Get User Drafts Request {:?}", user);

    match UserServiceClient::new(channel)
        .drafts(ArticlesRequest {
            cursor: cursor.cursor,
            limit: cursor.limit,
            username: user.username,
            by_user: Some(user.user_id),
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<FullArticle> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res
                        .articles
                        .iter()
                        .map(|article| FullArticle::from(article))
                        .collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn get_user_lists(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    Query(cursor): Query<CursorPagination>,
    Path(params): Path<PathParams>,
) -> Response {
    info!("Get User Lists Request {:?}", params);

    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();

    let username = match params.username {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    match UserServiceClient::new(channel)
        .lists(ListsRequest {
            cursor: cursor.cursor,
            limit: cursor.limit,
            username,
            by_user,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<List> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res.lists.iter().map(|list| List::from(list)).collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn get_user_serieses(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    Query(cursor): Query<CursorPagination>,
    Path(params): Path<PathParams>,
) -> Response {
    info!("Get User Serieses Request {:?}", params);

    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();

    let username = match params.username {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };

    match UserServiceClient::new(channel)
        .serieses(SeriesesRequest {
            cursor: cursor.cursor,
            limit: cursor.limit,
            username,
            by_user,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<Series> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res
                        .series
                        .iter()
                        .map(|series| Series::from(series))
                        .collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}
// #[derive(Debug, Deserialize)]
// pub struct CreateUserRequestBody {
//     pub username: String,
//     pub email: String,
//     pub email_verified: Option<DateTime<Utc>>,
//     pub image: Option<String>,
// }
//
// pub async fn post_user(
// State(_state): State<AppState>,
//     Json(payload): Json<CreateUserRequestBody>,
// ) -> Response {
//     match UserServiceClient::new(state.auth_server.clone())
//         .(GetUserRequest { username })
//         .await
//     {
//         Ok(res) => (StatusCode::OK, Json(json!(User::from(res.get_ref())))).into_response(),
//         Err(err) => {
//             error!("{:#?}", err);
//             let message = err.message().to_string();
//             let status_code = code_to_statudecode(err.code());
//             return (status_code, message).into_response();
//         }
//     }
// }

#[derive(Debug, Deserialize)]
pub struct PatchUserRequestBody {
    pub username: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
    pub urls: Option<Vec<String>>,
}

pub async fn patch_user(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Json(payload): Json<PatchUserRequestBody>,
) -> Response {
    info!("Update Users Request {:?} {:?}", user.user_id, payload);
    match UserServiceClient::new(channel)
        .update(UpdateRequest {
            id: user.user_id,
            username: payload.username,
            image: payload.image,
            bio: payload.bio,
            urls: payload.urls.unwrap_or_default(),
        })
        .await
    {
        Ok(res) => (StatusCode::OK, Json(json!(User::from(res.get_ref())))).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn follow(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let username = match params.username {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!("Follow User Request {:?} {:?}", user.user_id, username);
    match UserServiceClient::new(channel)
        .follow(FollowRequest {
            user_id: user.user_id,
            target: username,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn unfollow(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let username = match params.username {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!("Unfollow User Request {:?} {:?}", user.user_id, username);
    match UserServiceClient::new(channel)
        .unfollow(UnfollowRequest {
            user_id: user.user_id,
            target: username,
        })
        .await
    {
        Ok(res) => (StatusCode::OK, res.get_ref().message.to_owned()).into_response(),
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn get_followers(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    Query(cursor): Query<CursorPagination>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();

    // {
    //     Some(token) => match AccessToken::validate(token.token()) {
    //         Ok(token_payload) => token_payload,
    //         Err(error) => {
    //             error!("Unable to validate token: {:#?}", error);
    //             return (StatusCode::UNAUTHORIZED, "Verification failed").into_response();
    //         }
    //     },
    //     None => return (StatusCode::BAD_REQUEST, "No token").into_response(),
    // };

    let username = match params.username {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!("Get Followers Request {:?} {:?}", username, cursor);
    match UserServiceClient::new(channel)
        .followers(FollowersRequest {
            username,
            limit: cursor.limit,
            cursor: cursor.cursor,
            by_user,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<FullUser> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res.users.iter().map(|user| FullUser::from(user)).collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn get_following(
    headers: HeaderMap,
    Extension(channel): Extension<Channel>,
    Query(cursor): Query<CursorPagination>,
    State(_state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Response {
    let by_user = headers
        .typed_get::<Authorization<Bearer>>()
        .map(|token| {
            AccessToken::validate(token.token())
                .ok()
                .map(|token_payload| token_payload.payload.user_id.to_owned())
        })
        .flatten();
    let username = match params.username {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
    };
    info!("Get Following Request {:?} {:?}", username, cursor);
    match UserServiceClient::new(channel)
        .following(FollowingRequest {
            username,
            limit: cursor.limit,
            cursor: cursor.cursor,
            by_user,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<FullUser> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res.users.iter().map(|user| FullUser::from(user)).collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:#?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            return (status_code, message).into_response();
        }
    }
}

pub async fn get_feed(
    Extension(channel): Extension<Channel>,
    Extension(user): Extension<AccessTokenPayload>,
    Query(cursor): Query<CursorPagination>,
) -> Response {
    match UserServiceClient::new(channel)
        .feed(FeedRequest {
            cursor: cursor.cursor,
            limit: cursor.limit,
            user_id: user.user_id,
        })
        .await
    {
        Ok(res) => {
            let res = res.get_ref();
            (
                StatusCode::OK,
                Json(json!(ResultPaging::<FullArticle> {
                    next_cursor: res.next_cursor.to_owned(),
                    items: res
                        .articles
                        .iter()
                        .map(|article| FullArticle::from(article))
                        .collect()
                })),
            )
                .into_response()
        }
        Err(err) => {
            error!("{:?}", err);
            let message = err.message().to_string();
            let status_code = code_to_statudecode(err.code());
            (status_code, message).into_response()
        }
    }
}
// pub async fn delete_user(
// State(_state): State<AppState>,
//     Path(params): Path<PathParams>,
// ) -> Response {
//     let user_id = match params.user_id {
//         Some(v) => v,
//         None => return (StatusCode::BAD_REQUEST, "Wrong parameters").into_response(),
//     };
//     unimplemented!()
// }
