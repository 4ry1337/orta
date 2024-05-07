use chrono::{DateTime, Utc};

use crate::models::{
    article_model, comment_model, list_model, series_model, tag_model, user_model,
};
use crate::resource_proto::{Article, Comment, FullArticle, List, Series, Tag, User};

struct W<T>(T);

impl From<W<&DateTime<Utc>>> for Option<prost_types::Timestamp> {
    fn from(dt: W<&DateTime<Utc>>) -> Self {
        let dt = dt.0;
        Some(prost_types::Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        })
    }
}

impl From<W<Option<&DateTime<Utc>>>> for Option<prost_types::Timestamp> {
    fn from(dt: W<Option<&DateTime<Utc>>>) -> Self {
        let dt = dt.0;
        match dt {
            Some(dt) => Some(prost_types::Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }),
            None => None,
        }
    }
}

impl From<&user_model::User> for User {
    fn from(value: &user_model::User) -> Self {
        User {
            id: value.id,
            email: value.email.clone(),
            email_verified: W(value.email_verified.as_ref()).into(),
            username: value.username.clone(),
            image: value.image.clone(),
            role: value.role.to_string(),
            following_count: value.following_count,
            follower_count: value.follower_count,
            approved_at: W(value.approved_at.as_ref()).into(),
            deleted_at: W(value.deleted_at.as_ref()).into(),
        }
    }
}

impl From<&article_model::Article> for Article {
    fn from(value: &article_model::Article) -> Self {
        Article {
            id: value.id,
            title: value.title.clone(),
            slug: value.slug.clone(),
            like_count: value.like_count,
            comment_count: value.comment_count,
            created_at: W(&value.created_at).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
            published_at: W(value.published_at.as_ref()).into(),
        }
    }
}

impl From<&article_model::FullArticle> for FullArticle {
    fn from(value: &article_model::FullArticle) -> Self {
        FullArticle {
            id: value.id,
            title: value.title.clone(),
            slug: value.slug.clone(),
            like_count: value.like_count,
            comment_count: value.comment_count,
            created_at: W(&value.created_at).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
            published_at: W(value.published_at.as_ref()).into(),
            authors: match &value.authors {
                Some(authors) => authors.iter().map(|user| User::from(user)).collect(),
                None => vec![],
            },
            tags: match &value.tags {
                Some(tags) => tags.iter().map(|tag| Tag::from(tag)).collect(),
                None => vec![],
            },
        }
    }
}

impl From<&list_model::List> for List {
    fn from(value: &list_model::List) -> Self {
        List {
            id: value.id,
            user_id: value.user_id,
            label: value.label.clone(),
            slug: value.slug.clone(),
            image: value.image.clone(),
            visibility: value.visibility.to_string(),
            article_count: value.article_count,
            created_at: W(&value.created_at).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
        }
    }
}

impl From<&series_model::Series> for Series {
    fn from(value: &series_model::Series) -> Self {
        Series {
            id: value.id,
            user_id: value.user_id,
            label: value.label.clone(),
            slug: value.slug.clone(),
            image: value.image.clone(),
            article_count: value.article_count,
            created_at: W(&value.created_at).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
        }
    }
}

impl From<&comment_model::Comment> for Comment {
    fn from(value: &comment_model::Comment) -> Self {
        Comment {
            id: value.id,
            commenter_id: value.commenter_id,
            target_id: value.target_id,
            r#type: value.r#type.to_string(),
            content: value.content.clone(),
            created_at: W(&value.created_at).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
        }
    }
}

impl From<&tag_model::Tag> for Tag {
    fn from(value: &tag_model::Tag) -> Self {
        Tag {
            id: value.id,
            label: value.label.clone(),
            slug: value.slug.clone(),
            tag_status: value.tag_status.to_string(),
            article_count: value.article_count,
            created_at: W(&value.created_at).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
        }
    }
}
