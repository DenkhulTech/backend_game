// use diesel::prelude::*;
// use serde::{Deserialize, Serialize};
// use crate::schema::items;

// #[derive(Queryable, Insertable, Serialize, Deserialize)]
// #[diesel(table_name = items)]
// pub struct Item {
//     pub id: i32,
//     pub name: String,
//     pub r#type: String, // gunakan `r#type` karena "type" adalah keyword Rust
// }

// #[derive(Insertable)]
// #[diesel(table_name = items)]
// pub struct NewItem<'a> {
//     pub name: &'a str,
//     pub r#type: &'a str,
// }
