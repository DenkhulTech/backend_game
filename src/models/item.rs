use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,

    #[sqlx(rename = "type")] // <- Petakan ke kolom "type" di database
    #[serde(rename = "type")] // <- Jika kamu kirim via JSON
    pub type_: String,
}
