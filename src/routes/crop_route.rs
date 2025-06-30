use axum::{Router, routing::{get, post}};
use crate::controllers::crop_controller;
use crate::db::DbPool;

pub fn crop_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/crops", post(crop_controller::create_crop).get(crop_controller::get_all_crops))
        .route("/crops/{crop_id}", get(crop_controller::get_crop).put(crop_controller::update_crop).delete(crop_controller::delete_crop))
        .route("/crops/{crop_id}/stages", get(crop_controller::get_crop_stages))
        .route("/growth_stages", post(crop_controller::create_growth_stages))

        .with_state(pool)
}
