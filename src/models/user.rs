use serde::{Deserialize, Serialize};
use diesel::{prelude::*, SelectableHelper};
use crate::schema::users;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub wallet: Option<String>,
    pub session_id: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub wallet: Option<String>,
    pub session_id: String,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub wallet: Option<String>,
    pub session_id: Option<String>,
}