use axum::{
    Router,
    routing::{ post, put, get},
};
use crate::db::DbPool;

use crate::controllers::field_controller;

pub fn field_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/fields", post(field_controller::create_field).get(field_controller::get_all_fields))
        .route("/fields/{id}", put(field_controller::update_field).get(field_controller::get_field).delete(field_controller::delete_field))
        .route("/users/{user_id}/fields", 
            get(field_controller::get_user_fields)
        )
        .with_state(pool)
}
