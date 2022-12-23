use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}
