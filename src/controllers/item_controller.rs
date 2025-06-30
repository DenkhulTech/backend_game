// use axum::{extract::State, Json, http::StatusCode};
// use sqlx::PgPool;
// use crate::models::item::Item;
// use crate::services::item_service::get_all_items;

// pub async fn get_items_handler(
//     State(pool): State<PgPool>,
// ) -> Result<Json<Vec<Item>>, (StatusCode, String)> {
//     get_all_items(&pool)
//         .await
//         .map(Json)
//         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
// }
