use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Product2 {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize)]
pub struct Product2Catalog {
    pub products: Vec<Product2>,
}