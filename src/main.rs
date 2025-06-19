mod db;
mod models;
mod schema;

mod services;
mod controllers;
mod routes;

use axum::{Router};
use std::sync::{Arc, Mutex};
// use diesel::PgConnection;
use tower_http::cors::{CorsLayer, Any};
use routes::user_route::user_routes;

#[tokio::main]
async fn main() {
    // Setup database connection
    let conn = db::establish_connection();

     // 3. Bungkus koneksi untuk sharing thread-safe
    let conn_mutex = Arc::new(Mutex::new(conn));
    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application
    let app = Router::new()
        .nest("/api", user_routes(conn_mutex))
        .layer(cors);

    // Run server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}