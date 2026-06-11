use axum::{extract::State, Json};
use sqlx::PgPool;

use crate::models::product2::{Product2Catalog, Product2};

pub async fn get_product2_catalog(State(pool): State<PgPool>) -> Json<Product2Catalog> {
    let products = sqlx::query_as::<_, Product2>(
        "SELECT id, name FROM products WHERE category_id = 2",  
    )
    .fetch_all(&pool)
    .await
    .expect("failed to fetch products");

    Json(Product2Catalog { products })
}
