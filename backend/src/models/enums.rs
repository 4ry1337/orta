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

impl FromStr for Role {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<Role, Self::Err> {
        match input {
            "Admin" => Ok(Role::Admin),
            "User" => Ok(Role::User),
            "Manager" => Ok(Role::Manager),
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

impl FromStr for TagStatus {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<TagStatus, Self::Err> {
        match input {
            "Approved" => Ok(TagStatus::Approved),
            "Banned" => Ok(TagStatus::Banned),
            "Waiting" => Ok(TagStatus::Waiting),
            _ => Err(format!("Can not parse {} into Tag Status Enum", input).into()),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[sqlx(type_name = "Visibility", rename_all = "UPPERCASE")]
pub enum Visibility {
    Private,
    Public,
    Bulink,
}

impl FromStr for Visibility {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<Visibility, Self::Err> {
        match input {
            "Private" => Ok(Visibility::Private),
            "Public" => Ok(Visibility::Public),
            "Bulink" => Ok(Visibility::Bulink),
            _ => Err(format!("Can not parse {} into Visibility Enum", input).into()),
        }
    }
}
