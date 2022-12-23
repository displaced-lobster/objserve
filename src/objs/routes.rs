use axum::{
    extract::{Path, Query, State},
    Json,
};
use sqlx::sqlite::SqlitePool;

use crate::{errors::AppError, objs::Obj, Pagination};

pub async fn obj(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Obj>, AppError> {
    let obj = sqlx::query_as!(
        Obj,
        r#"
        SELECT *
        FROM objs
        WHERE id = ?1
        "#,
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(obj))
}

pub async fn objs(
    State(pool): State<SqlitePool>,
    pagination: Option<Query<Pagination>>,
) -> Result<Json<Vec<Obj>>, AppError> {
    let Query(pagination) = pagination.unwrap_or_default();
    let limit = pagination.limit.unwrap_or(u32::MAX);
    let offset = pagination.offset.unwrap_or(0);
    let objs = sqlx::query_as!(
        Obj,
        r#"
        SELECT *
        FROM objs
        ORDER BY name ASC
        LIMIT ?1 OFFSET ?2
        "#,
        limit,
        offset,
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(objs))
}
