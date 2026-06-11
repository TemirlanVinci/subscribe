use axum::{Router, routing::{get, post}};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod handlers;
mod models;

use handlers::catalog::get_catalog;
use handlers::product1::get_product1_catalog;
use handlers::product2::get_product2_catalog;
use handlers::specific_product::get_specific_product;
#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Переменная DATABASE_URL не найдена в .env");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Не удалось подключиться к базе данных");

    println!("✅ Успешное подключение к PostgreSQL!");

    let app = Router::new()
        .route("/api/catalog", get(get_catalog))
        .route("/api/catalog/product1", get(get_product1_catalog))
        .route("/api/catalog/product2", get(get_product2_catalog))
        .route("/api/catalog/specific_product", post(get_specific_product))
        .with_state(pool);

    // 3. Определяем адрес и порт (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Запуск сервера на http://{}", addr);

    // 4. Создаем слушатель TCP-протокола от Tokio
    let listener = TcpListener::bind(addr).await.unwrap();

    // 5. Запускаем сервер Axum, передавая ему слушателя и наши маршруты
    axum::serve(listener, app).await.unwrap();
}
