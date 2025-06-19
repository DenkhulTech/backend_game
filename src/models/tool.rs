// use diesel::prelude::*;
// use serde::{Deserialize, Serialize};
// use crate::schema::tools;
// use crate::models::item::Item;

// #[derive(Queryable, Insertable, Serialize, Deserialize)]
// #[diesel(table_name = tools)]
// #[diesel(belongs_to(Item, foreign_key = item_id))]
// pub struct Tool {
//     pub item_id: i32,
//     pub action: String,
//     pub durability: Option<i32>,
// }

// #[derive(Insertable)]
// #[diesel(table_name = tools)]
// pub struct NewTool<'a> {
//     pub item_id: i32,
//     pub action: &'a str,
//     pub durability: Option<i32>,
// }
