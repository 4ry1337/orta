use serde::{Deserialize, Serialize};

use crate::resource_proto::QueryParams;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultPaging<T> {
    pub metadata: Metadata,
    pub items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    total: i64,
    first_page: i64,
    last_page: i64,
    per_page: i64,
    page: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    pub order_by: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

// impl Metadata {
//     fn from(total: i64, page: i64, per_page: i64) -> Self {
//         let last_page = (total as f64 / per_page as f64).ceil() as i64;
//         Self {
//             total,
//             page,
//             per_page,
//             first_page: 1,
//             last_page,
//         }
//     }
// }

impl From<&Option<QueryParams>> for Filter {
    fn from(value: &Option<QueryParams>) -> Self {
        match value {
            Some(query_params) => {
                let page = query_params.page.unwrap_or(1);
                let per_page = query_params.per_page.unwrap_or(10);
                Self {
                    order_by: query_params.order_by.clone(),
                    limit: per_page,
                    offset: (page - 1) * per_page,
                }
            }
            None => Self {
                order_by: None,
                limit: 10,
                offset: 0,
            },
        }
    }
}

#[derive(Deserialize)]
pub struct PathParams {
    pub comment_id: Option<i32>,
    pub user_id: Option<i32>,
    pub article_id: Option<i32>,
    pub series_id: Option<i32>,
    pub tag_id: Option<i32>,
    pub list_id: Option<i32>,
}
