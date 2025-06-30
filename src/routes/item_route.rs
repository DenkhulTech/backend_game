// use axum::Router;
// use axum::routing::get;
// use crate::controllers::item_controller::get_items_handler;
// use sqlx::PgPool;

// pub fn item_routes(pool: PgPool) -> Router {
//     Router::new()
//         .route("/items", get(get_items_handler))
//         .with_state(pool)
// }
