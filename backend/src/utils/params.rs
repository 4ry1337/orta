use serde::{de, Deserialize, Deserializer, Serialize};
use serde_aux::prelude::default_i64;
use std::{fmt, str::FromStr};

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

#[derive(Debug, Serialize)]
pub struct ResultPaging<T> {
    pub pagination: Metadata,
    pub total: i64,
    pub items: Vec<T>,
}

//TODO: works?
#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_i64::<1>")]
    pub page: i64,
    #[serde(default = "default_i64::<25>")]
    pub per_page: i64,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub sort: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub query: Option<String>,
}

// impl Pagination {
//     pub fn per_page(&self) -> i64 {
//         self.per_page.0
//     }
// }

// #[derive(Debug, Deserialize)]
// pub struct PerPage(i64);
//
// impl Default for PerPage {
//     fn default() -> Self {
//         PerPage(CONFIG.query.per_page)
//     }
// }
//
// impl Default for Pagination {
//     fn default() -> Self {
//         Self {
//             page: 1,
//             per_page: CONFIG.query.per_page,
//             sort: None,
//             query: None,
//         }
//     }
// }

#[derive(Debug, Deserialize)]
pub struct PathParams {
    pub comment_id: Option<String>,
    pub user_id: Option<String>,
    pub asset_name: Option<String>,
    pub series_id: Option<String>,
    pub tag_id: Option<String>,
    pub list_id: Option<String>,
    pub username: Option<String>,
    pub asset: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Metadata {
    pub first_page: i64,
    pub last_page: i64,
    pub per_page: i64,
    pub page: i64,
}

impl Metadata {
    pub fn new(total: i64, per_page: i64, page: i64) -> Self {
        let last_page = (total as f64 / per_page as f64).ceil() as i64;
        Self {
            page,
            per_page,
            first_page: 1,
            last_page,
        }
    }
}
