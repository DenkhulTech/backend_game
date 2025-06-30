use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::models::crop::*;
use crate::schema::{crops::dsl::*, crop_growth_stages::dsl::*};
use diesel::PgConnection;
use anyhow::Result;

pub struct CropService;

impl CropService {
    pub fn create_crop(conn: &mut PgConnection, new_crop: NewCrop) -> Result<Crop> {
        diesel::insert_into(crops)
            .values(&new_crop)
            .returning(Crop::as_returning())
            .get_result(conn)
            .map_err(Into::into)
    }

    pub fn get_all_crops(conn: &mut PgConnection) -> Result<Vec<Crop>> {
        crops.select(Crop::as_select()).load(conn).map_err(Into::into)
    }

    pub fn get_crop(conn: &mut PgConnection, crop_id_filter: i32) -> Result<Option<Crop>> {
        crops.find(crop_id_filter).select(Crop::as_select()).first(conn).optional().map_err(Into::into)
    }

    pub fn update_crop(conn: &mut PgConnection, crop_id_filter: i32, update: UpdateCrop) -> Result<Crop> {
        diesel::update(crops.find(crop_id_filter))
            .set(update)
            .returning(Crop::as_select())
            .get_result(conn)
            .map_err(Into::into)
    }

    pub fn delete_crop(conn: &mut PgConnection, crop_id_filter: i32) -> Result<usize> {
        diesel::delete(crops.find(crop_id_filter)).execute(conn).map_err(Into::into)
    }

    pub fn get_growth_stages(conn: &mut PgConnection, crop_id_filter: i32) -> Result<Vec<CropGrowthStage>> {
        crop_growth_stages
            .filter(crop_id.eq(crop_id_filter))
            .select(CropGrowthStage::as_select())
            .order(stage_index.asc())
            .load(conn)
            .map_err(Into::into)
    }

    pub fn add_growth_stages(
    conn: &mut PgConnection,
    stages: Vec<NewCropGrowthStage>,
    ) -> Result<usize> {
    diesel::insert_into(crop_growth_stages)
        .values(&stages)
        .execute(conn)
        .map_err(Into::into)
    }
   pub fn get_crop_with_stages(conn: &mut PgConnection, crop_id_value: i32) -> Result<Option<CropWithStages>> {
    let crop = crops
        .find(crop_id_value)
        .select(Crop::as_select())
        .first(conn)
        .optional()?;

    if let Some(crop) = crop {
        let stages = crop_growth_stages
            .filter(crop_id.eq(crop.id))
            .order(stage_index.asc())
            .select(CropGrowthStage::as_select())
            .load::<CropGrowthStage>(conn)?;

        Ok(Some(CropWithStages {
            id: crop.id,
            name: crop.name,
            yield_item: crop.yield_item,
            sell_price: crop.sell_price,
            growth_stages: stages,
        }))
    } else {
        Ok(None)
    }
}


}
