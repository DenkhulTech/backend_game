// use crate::schema::crops;
// use diesel::{Queryable, Insertable};
// use serde::{Deserialize, Serialize};
// use std::time::SystemTime;

// #[derive(Queryable, Insertable, Serialize, Deserialize)]
// #[diesel(table_name = crops)]
// pub struct Crop {
//     pub id: i32,
//     pub name: String,
//     pub grow_time: i32,
//     pub yield_item: i32,
//     pub sell_price: i32,
// }

// #[derive(Insertable, Deserialize, Serialize)]
// #[diesel(table_name = crops)]
// pub struct NewCrop<'a> {
//     pub name: &'a str,
//     pub grow_time: i32,
//     pub yield_item: i32,
//     pub sell_price: i32,
// }
