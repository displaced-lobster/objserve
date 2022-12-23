use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub src: String,
}
