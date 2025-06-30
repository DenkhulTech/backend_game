use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::{db::DbPool, models::crop::*, services::crop_service::CropService};

// Buat crop baru
pub async fn create_crop(State(pool): State<DbPool>, Json(new_crop): Json<NewCrop>) -> Result<(StatusCode, Json<Crop>), (StatusCode, String)> {
    let mut conn = pool.get().unwrap();
    CropService::create_crop(&mut conn, new_crop)
        .map(|c| (StatusCode::CREATED, Json(c)))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Ambil semua crops
pub async fn get_all_crops(State(pool): State<DbPool>) -> Result<Json<Vec<Crop>>, (StatusCode, String)> {
    let mut conn = pool.get().unwrap();
    CropService::get_all_crops(&mut conn)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Ambil crop tunggal
pub async fn get_crop(State(pool): State<DbPool>, Path(crop_id): Path<i32>) -> Result<Json<CropWithStages>, (StatusCode, String)> {
    let mut conn = pool.get().unwrap();
    match CropService::get_crop_with_stages(&mut conn, crop_id) {
        Ok(Some(crop)) => Ok(Json(crop)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Crop not found".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

// Update crop
pub async fn update_crop(State(pool): State<DbPool>, Path(crop_id): Path<i32>, Json(update): Json<UpdateCrop>) -> Result<Json<Crop>, (StatusCode, String)> {
    let mut conn = pool.get().unwrap();
    CropService::update_crop(&mut conn, crop_id, update)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Delete crop
pub async fn delete_crop(State(pool): State<DbPool>, Path(crop_id): Path<i32>) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = pool.get().unwrap();
    CropService::delete_crop(&mut conn, crop_id)
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// Ambil growth stages per crop
pub async fn get_crop_stages(State(pool): State<DbPool>, Path(crop_id): Path<i32>) -> Result<Json<Vec<CropGrowthStage>>, (StatusCode, String)> {
    let mut conn = pool.get().unwrap();
    CropService::get_growth_stages(&mut conn, crop_id)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn create_growth_stages(
    State(pool): State<DbPool>,
    Json(stages): Json<Vec<NewCropGrowthStage>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut conn = pool.get().unwrap();
    match CropService::add_growth_stages(&mut conn, stages) {
        Ok(count) => Ok((StatusCode::CREATED, format!("Inserted {} stages", count))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
