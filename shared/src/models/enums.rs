use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[sqlx(type_name = "role", rename_all = "UPPERCASE")]
pub enum Role {
    Admin,
    User,
    Manager,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "ADMIN"),
            Role::User => write!(f, "USER"),
            Role::Manager => write!(f, "MANAGER"),
        }
    }
}

impl FromStr for Role {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<Role, Self::Err> {
        match input.to_lowercase().trim() {
            "ADMIN" => Ok(Role::Admin),
            "USER" => Ok(Role::User),
            "MANAGER" => Ok(Role::Manager),
            _ => Err(format!("Can not parse {} into Role Enum", input).into()),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[sqlx(type_name = "TagStatus", rename_all = "UPPERCASE")]
pub enum TagStatus {
    Approved,
    Banned,
    Waiting,
}

impl std::fmt::Display for TagStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TagStatus::Approved => write!(f, "APPROVED"),
            TagStatus::Banned => write!(f, "BANNED"),
            TagStatus::Waiting => write!(f, "WAITING"),
        }
    }
}

impl FromStr for TagStatus {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<TagStatus, Self::Err> {
        match input.to_lowercase().trim() {
            "APPROVED" => Ok(TagStatus::Approved),
            "BANNED" => Ok(TagStatus::Banned),
            "WAITING" => Ok(TagStatus::Waiting),
            _ => Err(format!("Can not parse {} into Tag Status Enum", input).into()),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[sqlx(type_name = "Visibility", rename_all = "UPPERCASE")]
pub enum Visibility {
    Private,
    Public,
    Bylink,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Visibility::Private => write!(f, "PRIVATE"),
            Visibility::Public => write!(f, "PUBLIC"),
            Visibility::Bylink => write!(f, "BYLINK"),
        }
    }
}

impl FromStr for Visibility {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<Visibility, Self::Err> {
        match input.to_lowercase().trim() {
            "PRIVATE" => Ok(Visibility::Private),
            "PUBLIC" => Ok(Visibility::Public),
            "BYLINK" => Ok(Visibility::Bylink),
            _ => Err(format!("Can not parse {} into Visibility Enum", input).into()),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[sqlx(type_name = "CommentableType", rename_all = "UPPERCASE")]
pub enum CommentableType {
    Article,
    List,
    Series,
}

impl std::fmt::Display for CommentableType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommentableType::Article => write!(f, "ARTICLE"),
            CommentableType::List => write!(f, "LIST"),
            CommentableType::Series => write!(f, "SERIES"),
        }
    }
}

impl FromStr for CommentableType {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<CommentableType, Self::Err> {
        match input.to_lowercase().trim() {
            "ARTICLE" => Ok(CommentableType::Article),
            "LIST" => Ok(CommentableType::List),
            "SERIES" => Ok(CommentableType::Series),
            _ => Err(format!("Can not parse {} into CommentableType Enum", input).into()),
        }
    }
}
