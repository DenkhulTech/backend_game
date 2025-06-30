

use crate::models::field::{Field, NewField, UpdateField};
use crate::services::field_service::FieldService;
use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use crate::db::DbPool;


// Create Field
pub async fn create_field(
    State(pool): State<DbPool>,
    Json(new_field): Json<NewField>,
) -> Result<(StatusCode, Json<Field>), (StatusCode, String)> {
    let mut conn = pool.get().expect("DB pool error");
    FieldService::create_field(&mut *conn, new_field)
        .map(|field| (StatusCode::CREATED, Json(field)))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Get All Fields
pub async fn get_all_fields(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Field>>, (StatusCode, String)> {
    let mut conn = pool.get().expect("DB pool error");
    FieldService::get_all_fields(&mut *conn)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Get Single Field
pub async fn get_field(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Field>, (StatusCode, String)> {
    let mut conn = pool.get().expect("DB pool error");
    match FieldService::get_field(&mut *conn, user_id) {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// Update Field
pub async fn update_field(
    State(pool): State<DbPool>,
    Path(field_id): Path<i32>,
    Json(update_data): Json<UpdateField>,
) -> Result<Json<Field>, (StatusCode, String)> {
    println!("🔧 [UPDATE FIELD] Masuk handler, field_id = {}", field_id);
    println!("📦 Data diterima: {:?}", update_data);

    let mut conn = match pool.get() {
        Ok(c) => {
            println!("✅ Koneksi DB berhasil");
            c
        },
        Err(e) => {
            println!("❌ Gagal ambil koneksi DB: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "DB connection error".into()));
        }
    };

    match FieldService::update_field(&mut conn, field_id, update_data) {
        Ok(updated_field) => {
            println!("✅ Update sukses: {:?}", updated_field);
            Ok(Json(updated_field))
        },
        Err(e) => {
            println!("❌ Update gagal: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}
// Delete Field
pub async fn delete_field(
    State(pool): State<DbPool>,
    Path(field_id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = pool.get().expect("DB pool error");
    FieldService::delete_field(&mut *conn, field_id)
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Get Fields by User ID
pub async fn get_user_fields(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Field>>, (StatusCode, String)> {
    let mut conn = pool.get().expect("DB pool error");
    FieldService::get_fields_by_user(&mut *conn, user_id)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}