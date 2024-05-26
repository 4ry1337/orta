use serde::{Deserialize, Serialize};
use serde_aux::prelude::default_i64;

// /// Serde deserialization decorator to map empty Strings to None,
// fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
// where
//     D: Deserializer<'de>,
//     T: FromStr,
//     T::Err: fmt::Display,
// {
//     let opt = Option::<String>::deserialize(de)?;
//     match opt.as_deref() {
//         None | Some("") => Ok(None),
//         Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
//     }
// }

#[derive(Debug, Serialize)]
pub struct ResultPaging<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CursorPagination {
    pub cursor: Option<String>,
    #[serde(default = "default_i64::<25>")]
    pub limit: i64,
}

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
