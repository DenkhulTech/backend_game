use diesel::{ PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,OptionalExtension};
use crate::models::user::{User, NewUser, UpdateUser};
use crate::schema::users::dsl::*;
use anyhow::Result;

pub struct UserService;

impl UserService {
    // Create
    pub fn create_user(
        conn: &mut PgConnection,
        new_user: NewUser,
    ) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users)
            .values(&new_user)
            .returning(User::as_select())
            .get_result(conn)
    }

    // Get All
    pub fn get_all_users(
        conn: &mut PgConnection,
    ) -> Result<Vec<User>, diesel::result::Error> {
        users
            .select(User::as_returning())
            .load(conn)
    }

    // Get Single
pub fn get_user(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<Option<User>, diesel::result::Error> {
    users
        .find(user_id)
        .select(User::as_select()) // ⛔ butuh derive Selectable
        .first::<User>(conn)
        .optional()
}

    // Update
    pub fn update_user(
        conn: &mut PgConnection,
        user_id: i32,
        update_data: UpdateUser,
    ) -> Result<User, diesel::result::Error> {
        diesel::update(users.find(user_id))
            .set(&update_data)
            .returning(User::as_select())
            .get_result(conn)
    }

    // Delete
    pub fn delete_user(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(users.find(user_id))
            .execute(conn)
    }
}