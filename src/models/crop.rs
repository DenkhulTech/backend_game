use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::{crops, crop_growth_stages};

#[derive(Queryable, Serialize, Debug, Clone, Selectable)]
#[diesel(table_name = crops)]
pub struct Crop {
    pub id: i32,
    pub name: String,
    pub yield_item: i32,
    pub sell_price: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crops)]
pub struct NewCrop {
    pub name: String,
    pub yield_item: i32,
    pub sell_price: i32,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crops)]
pub struct UpdateCrop {
    pub name: Option<String>,
    pub yield_item: Option<i32>,
    pub sell_price: Option<i32>,
}

#[derive(Queryable, Serialize, Debug, Clone, Selectable)]
#[diesel(table_name = crop_growth_stages)]
pub struct CropGrowthStage {
    pub id: i32,
    pub crop_id: i32,
    pub stage_index: i32,
    pub stage_name: String,
    pub duration_seconds: i32,
    pub sprite_url: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crop_growth_stages)]
pub struct NewCropGrowthStage {
    pub crop_id: i32,
    pub stage_index: i32,
    pub stage_name: String,
    pub duration_seconds: i32,
    pub sprite_url: String,
}

#[derive(Serialize)]
pub struct CropWithStages {
    pub id: i32,
    pub name: String,
    pub yield_item: i32,
    pub sell_price: i32,
    pub growth_stages: Vec<CropGrowthStage>,
}

