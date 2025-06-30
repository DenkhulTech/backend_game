use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::fields;
use crate::models::user::User;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable, Associations)]
#[diesel(table_name = fields)]
#[diesel(belongs_to(User, foreign_key = user_id))]  // Tambahkan relasi
pub struct Field {
    pub id: i32,
    pub user_id: Option<i32>,  // Foreign key ke users
    pub tile_x: i32,
    pub tile_y: i32,
    pub is_tilled: bool,
    pub is_watered: bool,
    pub crop_id: Option<i32>,
    pub plant_time: Option<chrono::NaiveDateTime>,
    pub ready_time: Option<chrono::NaiveDateTime>,
    pub status: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = fields)]
pub struct NewField {
    pub user_id: Option<i32>,
    pub tile_x: i32,
    pub tile_y: i32,
    pub is_tilled: bool,
    pub is_watered: bool,
    pub crop_id: Option<i32>,
    pub plant_time: Option<chrono::NaiveDateTime>,
    pub ready_time: Option<chrono::NaiveDateTime>,
    pub status: String,
}
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = fields)]
pub struct UpdateField {
    pub user_id: Option<i32>,
    pub is_tilled: Option<bool>,
    pub is_watered: Option<bool>,
    pub crop_id: Option<i32>,
    pub plant_time: Option<NaiveDateTime>,
    pub ready_time: Option<NaiveDateTime>,
    pub status: Option<String>,
}