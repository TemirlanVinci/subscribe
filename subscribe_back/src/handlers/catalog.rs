use axum::{extract::State, Json};
use sqlx::PgPool;

use crate::models::catalog::{Catalog, Category};

pub async fn get_catalog(State(pool): State<PgPool>) -> Json<Catalog> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT id, name FROM categories ORDER BY id",
    )
    .fetch_all(&pool)
    .await
    .expect("failed to fetch categories");

    Json(Catalog { categories })
}
