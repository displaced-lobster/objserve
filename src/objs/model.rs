use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct Obj {
    pub id: i64,
    pub collection: Option<i64>,
    pub name: String,
    pub src: String,
    pub thumbnail: Option<String>,
}
