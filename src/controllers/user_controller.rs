use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use std::sync::{Arc, Mutex};
use diesel::PgConnection;
use crate::models::user::{User, NewUser, UpdateUser};
use crate::services::user_service::UserService;

// Create User
pub async fn create_user(
    State(conn): State<Arc<Mutex<PgConnection>>>,
    Json(new_user): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let mut conn = conn.lock().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    UserService::create_user(&mut *conn, new_user)
        .map(|user| (StatusCode::CREATED, Json(user)))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Get All Users
pub async fn get_all_users(
    State(conn): State<Arc<Mutex<PgConnection>>>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let mut conn = conn.lock().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    UserService::get_all_users(&mut *conn)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Get Single User
// pub async fn get_user(
//     State(conn): State<Arc<Mutex<PgConnection>>>,
//     Path(user_id): Path<i32>,
// ) -> Result<Json<User>, (StatusCode, String)> {
//     let mut conn = conn.lock().map_err(|e| {
//         (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
//     })?;

//     match UserService::get_user(&mut *conn, user_id) {
//         Ok(Some(user)) => Ok(Json(user)),
//         Ok(None)) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
//         Err(e)) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
//     }
// }

// Update User
pub async fn update_user(
    State(conn): State<Arc<Mutex<PgConnection>>>,
    Path(user_id): Path<i32>,
    Json(update_data): Json<UpdateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = conn.lock().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    UserService::update_user(&mut *conn, user_id, update_data)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Delete User
pub async fn delete_user(
    State(conn): State<Arc<Mutex<PgConnection>>>,
    Path(user_id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = conn.lock().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    UserService::delete_user(&mut *conn, user_id)
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}