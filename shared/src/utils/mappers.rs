use chrono::{DateTime, Utc};

use crate::models::{
    article_model, comment_model, enums, list_model, series_model, tag_model, user_model,
};
use crate::resource_proto::{
    Article, ArticleVersion, Comment, CommentableType, FullArticle, List, Role, Series, Tag,
    TagStatus, User, Visibility,
};

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

impl From<W<Option<&prost_types::Timestamp>>> for Option<DateTime<Utc>> {
    fn from(dt: W<Option<&prost_types::Timestamp>>) -> Self {
        let dt = dt.0;
        match dt {
            Some(dt) => DateTime::from_timestamp(dt.seconds, dt.nanos.try_into().unwrap()),
            None => None,
        }
    }
}

impl From<W<Option<&prost_types::Timestamp>>> for DateTime<Utc> {
    fn from(dt: W<Option<&prost_types::Timestamp>>) -> Self {
        let dt = dt.0.unwrap();
        DateTime::from_timestamp(dt.seconds, dt.nanos.try_into().unwrap()).unwrap()
    }
}

impl From<enums::Role> for Role {
    fn from(value: enums::Role) -> Self {
        match value {
            enums::Role::User => Self::User,
            enums::Role::Admin => Self::Admin,
            enums::Role::Manager => Self::Manager,
        }
    }
}

impl From<Role> for enums::Role {
    fn from(value: Role) -> Self {
        match value {
            Role::User => Self::User,
            Role::Admin => Self::Admin,
            Role::Manager => Self::Manager,
        }
    }
}

impl From<enums::TagStatus> for TagStatus {
    fn from(value: enums::TagStatus) -> Self {
        match value {
            enums::TagStatus::Approved => Self::Approved,
            enums::TagStatus::Waiting => Self::Waiting,
            enums::TagStatus::Banned => Self::Banned,
        }
    }
}

impl From<TagStatus> for enums::TagStatus {
    fn from(value: TagStatus) -> Self {
        match value {
            TagStatus::Approved => Self::Approved,
            TagStatus::Waiting => Self::Waiting,
            TagStatus::Banned => Self::Banned,
        }
    }
}

impl From<enums::Visibility> for Visibility {
    fn from(value: enums::Visibility) -> Self {
        match value {
            enums::Visibility::Public => Self::Public,
            enums::Visibility::Private => Self::Private,
            enums::Visibility::Bylink => Self::Bylink,
        }
    }
}

impl From<Visibility> for enums::Visibility {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::Public => Self::Public,
            Visibility::Private => Self::Private,
            Visibility::Bylink => Self::Bylink,
        }
    }
}

impl From<enums::CommentableType> for CommentableType {
    fn from(value: enums::CommentableType) -> Self {
        match value {
            enums::CommentableType::Article => Self::Article,
            enums::CommentableType::List => Self::List,
            enums::CommentableType::Series => Self::Series,
        }
    }
}

impl From<CommentableType> for enums::CommentableType {
    fn from(value: CommentableType) -> Self {
        match value {
            CommentableType::Article => Self::Article,
            CommentableType::List => Self::List,
            CommentableType::Series => Self::Series,
        }
    }
}

impl From<&user_model::User> for User {
    fn from(value: &user_model::User) -> Self {
        User {
            id: value.id.clone(),
            email: value.email.clone(),
            email_verified: W(value.email_verified.as_ref()).into(),
            username: value.username.clone(),
            image: value.image.clone(),
            role: Role::from(value.role) as i32,
            bio: value.bio.clone(),
            urls: value.urls.clone(),
            following_count: value.following_count,
            follower_count: value.follower_count,
            created_at: W(&value.created_at).into(),
            approved_at: W(value.approved_at.as_ref()).into(),
            deleted_at: W(value.deleted_at.as_ref()).into(),
        }
    }
}

impl From<&User> for user_model::User {
    fn from(value: &User) -> Self {
        user_model::User {
            id: value.id.clone(),
            email: value.email.clone(),
            email_verified: W(value.email_verified.as_ref()).into(),
            username: value.username.clone(),
            image: value.image.clone(),
            role: value.role().into(),
            bio: value.bio.clone(),
            urls: value.urls.clone(),
            following_count: value.following_count,
            follower_count: value.follower_count,
            created_at: W(value.created_at.as_ref()).into(),
            approved_at: W(value.approved_at.as_ref()).into(),
            deleted_at: W(value.deleted_at.as_ref()).into(),
        }
    }
}

impl From<&article_model::Article> for Article {
    fn from(value: &article_model::Article) -> Self {
        Article {
            id: value.id.clone(),
            title: value.title.clone(),
            // content: value.content.clone(),
            like_count: value.like_count,
            comment_count: value.comment_count,
            created_at: W(&value.created_at).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
            published_at: W(value.published_at.as_ref()).into(),
        }
    }
}

impl From<&Article> for article_model::Article {
    fn from(value: &Article) -> Self {
        article_model::Article {
            id: value.id.clone(),
            title: value.title.clone(),
            // content: value.content.clone(),
            like_count: value.like_count,
            comment_count: value.comment_count,
            created_at: W(value.created_at.as_ref()).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
            published_at: W(value.published_at.as_ref()).into(),
        }
    }
}

impl From<&article_model::FullArticle> for FullArticle {
    fn from(value: &article_model::FullArticle) -> Self {
        FullArticle {
            id: value.id.clone(),
            title: value.title.clone(),
            like_count: value.like_count,
            content: value.content.clone(),
            comment_count: value.comment_count,
            created_at: W(&value.created_at).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
            published_at: W(value.published_at.as_ref()).into(),
            users: match &value.users {
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

impl From<&FullArticle> for article_model::FullArticle {
    fn from(value: &FullArticle) -> Self {
        article_model::FullArticle {
            id: value.id.clone(),
            title: value.title.clone(),
            like_count: value.like_count,
            content: value.content.clone(),
            comment_count: value.comment_count,
            created_at: W(value.created_at.as_ref()).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
            published_at: W(value.published_at.as_ref()).into(),
            users: Some(
                value
                    .users
                    .iter()
                    .map(|user| user_model::User::from(user))
                    .collect(),
            ),
            tags: Some(
                value
                    .tags
                    .iter()
                    .map(|tag| tag_model::Tag::from(tag))
                    .collect(),
            ),
        }
    }
}

impl From<&list_model::List> for List {
    fn from(value: &list_model::List) -> Self {
        List {
            id: value.id.clone(),
            user_id: value.user_id.clone(),
            label: value.label.clone(),
            image: value.image.clone(),
            visibility: Visibility::from(value.visibility) as i32,
            article_count: value.article_count,
            created_at: W(&value.created_at).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
        }
    }
}

impl From<&List> for list_model::List {
    fn from(value: &List) -> Self {
        list_model::List {
            id: value.id.clone(),
            user_id: value.user_id.clone(),
            label: value.label.clone(),
            image: value.image.clone(),
            visibility: enums::Visibility::from(value.visibility()),
            article_count: value.article_count,
            created_at: W(value.created_at.as_ref()).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
        }
    }
}

impl From<&series_model::Series> for Series {
    fn from(value: &series_model::Series) -> Self {
        Series {
            id: value.id.clone(),
            user_id: value.user_id.clone(),
            label: value.label.clone(),
            image: value.image.clone(),
            article_count: value.article_count,
            created_at: W(&value.created_at).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
        }
    }
}

impl From<&Series> for series_model::Series {
    fn from(value: &Series) -> Self {
        series_model::Series {
            id: value.id.clone(),
            user_id: value.user_id.clone(),
            label: value.label.clone(),
            image: value.image.clone(),
            article_count: value.article_count,
            created_at: W(value.created_at.as_ref()).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
        }
    }
}

impl From<&comment_model::Comment> for Comment {
    fn from(value: &comment_model::Comment) -> Self {
        Comment {
            id: value.id.clone(),
            commenter_id: value.commenter_id.clone(),
            target_id: value.target_id.clone(),
            r#type: CommentableType::from(value.r#type) as i32,
            content: value.content.clone(),
            created_at: W(&value.created_at).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
        }
    }
}

impl From<&Comment> for comment_model::Comment {
    fn from(value: &Comment) -> Self {
        Self {
            id: value.id.clone(),
            commenter_id: value.commenter_id.clone(),
            target_id: value.target_id.clone(),
            r#type: value.r#type().into(),
            content: value.content.clone(),
            created_at: W(value.created_at.as_ref()).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
        }
    }
}

impl From<&tag_model::Tag> for Tag {
    fn from(value: &tag_model::Tag) -> Self {
        Tag {
            slug: value.slug.clone(),
            label: value.label.clone(),
            tag_status: TagStatus::from(value.tag_status) as i32,
            article_count: value.article_count,
            created_at: W(&value.created_at).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
        }
    }
}

impl From<&Tag> for tag_model::Tag {
    fn from(value: &Tag) -> Self {
        Self {
            slug: value.slug.clone(),
            label: value.label.clone(),
            tag_status: value.tag_status().into(),
            article_count: value.article_count,
            created_at: W(value.created_at.as_ref()).into(),
            updated_at: W(value.updated_at.as_ref()).into(),
        }
    }
}

impl From<&article_model::ArticleVersion> for ArticleVersion {
    fn from(value: &article_model::ArticleVersion) -> Self {
        Self {
            id: value.id.clone(),
            article_id: value.article_id.clone(),
            device_id: value.device_id.clone(),
            content: value.content.clone(),
            created_at: W(&value.created_at).into(),
        }
    }
}

impl From<&ArticleVersion> for article_model::ArticleVersion {
    fn from(value: &ArticleVersion) -> Self {
        Self {
            id: value.id.clone(),
            article_id: value.article_id.clone(),
            device_id: value.device_id.clone(),
            content: value.content.clone(),
            created_at: W(value.created_at.as_ref()).into(),
        }
    }
}
