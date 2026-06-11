use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

use crate::models::specific_product::{SpecificProduct, SpecificProductId};

pub async fn get_specific_product(
    State(pool): State<PgPool>,
    Json(payload): Json<SpecificProductId>,
) -> Result<Json<SpecificProduct>, StatusCode> {
    let product = sqlx::query_as::<_, SpecificProduct>(
        "SELECT name, description, price FROM products WHERE id = $1",
    )
    .bind(payload.id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(product))
}
