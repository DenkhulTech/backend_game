// use diesel::prelude::*;
// use serde::{Deserialize, Serialize};
// use crate::schema::fields;
// use crate::models::user::User;

// #[derive(Queryable, Insertable, Serialize, Deserialize)]
// #[diesel(table_name = fields)]
// #[diesel(belongs_to(User, foreign_key = user_id))]
// pub struct Field {
//     pub id: i32,
//     pub user_id: Option<i32>,
//     pub tile_x: i32,
//     pub tile_y: i32,
//     pub is_tilled: bool,
//     pub is_watered: bool,
//     pub crop_id: Option<i32>,
//     pub plant_time: Option<chrono::NaiveDateTime>,
//     pub ready_time: Option<chrono::NaiveDateTime>,
//     pub status: String,
// }

// #[derive(Insertable)]
// #[diesel(table_name = fields)]
// pub struct NewField {
//     pub user_id: Option<i32>,
//     pub tile_x: i32,
//     pub tile_y: i32,
//     pub is_tilled: bool,
//     pub is_watered: bool,
//     pub crop_id: Option<i32>,
//     pub plant_time: Option<chrono::NaiveDateTime>,
//     pub ready_time: Option<chrono::NaiveDateTime>,
//     pub status: String,
// }
