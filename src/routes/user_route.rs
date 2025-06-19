use axum::{
    Router,
    routing::{get, post, put, delete},
};
use std::sync::{Arc, Mutex};
use diesel::PgConnection;
use crate::controllers::user_controller;

pub fn user_routes(conn: Arc<Mutex<PgConnection>>) -> Router {
    Router::new()
        .route("/users", post(user_controller::create_user).get(user_controller::get_all_users))
        .route("/users/{id}", put(user_controller::update_user).delete(user_controller::delete_user))
        .with_state(conn)
}