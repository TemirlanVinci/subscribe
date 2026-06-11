use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Product1 {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize)]
pub struct Product1Catalog {
    pub products: Vec<Product1>,
}