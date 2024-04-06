#[derive(sqlx::FromRow, Debug)]
pub struct Password {
    pub id: i32,
    pub password: String,
    pub salt: String,
}
