mod db;
mod models;
mod schema;
mod services;
mod controllers;
mod routes;

use axum::Router;
use routes::user_route::user_routes;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    // Ganti establish_connection dengan init_pool (r2d2)
    let pool = db::init_pool();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", user_routes(pool.clone())) // clone pool per route
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
