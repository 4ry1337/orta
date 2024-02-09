use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub id: Option<String>,
    pub label: String,
    pub image: Option<String>,
    pub visibility: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
