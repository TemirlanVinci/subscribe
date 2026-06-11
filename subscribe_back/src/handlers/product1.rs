use axum::{extract::State, Json};
use sqlx::PgPool;

use crate::models::product1::{Product1Catalog, Product1};

pub async fn get_product1_catalog(State(pool): State<PgPool>) -> Json<Product1Catalog> {
    let products = sqlx::query_as::<_, Product1>(
        "SELECT id, name FROM products WHERE category_id = 1",  
    )
    .fetch_all(&pool)
    .await
    .expect("failed to fetch products");

    Json(Product1Catalog { products })
}
