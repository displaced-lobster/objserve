use axum::{
    extract::{Path, Query, State},
    Json,
};
use sqlx::sqlite::SqlitePool;

use crate::{errors::AppError, Collection, Obj, Pagination};

pub async fn collection_objs(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Obj>>, AppError> {
    let objs = sqlx::query_as!(
        Obj,
        r#"
        SELECT *
        FROM objs
        WHERE collection = ?1
        "#,
        id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(objs))
}

pub async fn collections(
    State(pool): State<SqlitePool>,
    pagination: Option<Query<Pagination>>,
) -> Result<Json<Vec<Collection>>, AppError> {
    let Query(pagination) = pagination.unwrap_or_default();
    let limit = pagination.limit.unwrap_or(u32::MAX);
    let offset = pagination.offset.unwrap_or(0);
    let collections = sqlx::query_as!(
        Collection,
        r#"
        SELECT *
        FROM collections
        ORDER BY id ASC
        LIMIT ?1 OFFSET ?2
        "#,
        limit,
        offset,
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(collections))
}
