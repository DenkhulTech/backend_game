use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use crate::db::DbPool;
use crate::models::user::{User, NewUser, UpdateUser};
use crate::services::user_service::UserService;
use tokio::task;

// Create User
pub async fn create_user(
    State(pool): State<DbPool>,
    Json(new_user): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let mut conn = pool.get().expect("DB pool error");

    let result = task::spawn_blocking(move || {
        UserService::create_user(&mut conn, new_user)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    result.map(|user| (StatusCode::CREATED, Json(user)))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Get All Users
pub async fn get_all_users(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let mut conn = pool.get().expect("DB pool error");

    let result = task::spawn_blocking(move || {
        UserService::get_all_users(&mut conn)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    result.map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Update User
pub async fn update_user(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(update_data): Json<UpdateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().expect("DB pool error");

    let result = task::spawn_blocking(move || {
        UserService::update_user(&mut conn, user_id, update_data)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    result.map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Delete User
pub async fn delete_user(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = pool.get().expect("DB pool error");

    let result = task::spawn_blocking(move || {
        UserService::delete_user(&mut conn, user_id)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    result.map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}