mod db;
mod models;
mod schema;
mod services;
mod controllers;
mod routes;

use axum::Router;
use routes::user_route::user_routes;
use routes::field_route::field_routes;
use routes::crop_route::crop_routes;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    // Inisialisasi koneksi database pool
    let pool = db::init_pool();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Gabungkan semua routes di bawah /api
    let app = Router::new()
        .nest("/api", user_routes(pool.clone()))
        .nest("/api", field_routes(pool.clone()))
        .nest("/api", crop_routes(pool.clone()))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
