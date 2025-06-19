// use diesel::prelude::*;
// use serde::{Deserialize, Serialize};
// use crate::schema::inventory_items;
// use crate::models::user::User;

// #[derive(Queryable, Insertable, Serialize, Deserialize)]
// #[diesel(table_name = inventory_items)]
// #[diesel(belongs_to(User, foreign_key = user_id))]
// pub struct InventoryItem {
//     pub id: i32,
//     pub user_id: Option<i32>,
//     pub item_id: i32,
//     pub quantity: i32,
// }

// #[derive(Insertable)]
// #[diesel(table_name = inventory_items)]
// pub struct NewInventoryItem {
//     pub user_id: Option<i32>,
//     pub item_id: i32,
//     pub quantity: i32,
// }
