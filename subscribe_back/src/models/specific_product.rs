use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct SpecificProductId {
    pub id: i32,
}

#[derive(Serialize, FromRow)]
pub struct SpecificProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
}
