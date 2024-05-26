pub struct List {
    pub id: String,
    pub user_id: String,
    pub label: String,
    pub image: Option<String>,
    pub visibility: Visibility,
    pub article_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
