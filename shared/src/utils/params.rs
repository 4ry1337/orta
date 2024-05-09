use serde::{Deserialize, Serialize};

use crate::resource_proto::QueryParams;

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    pub order_by: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

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
