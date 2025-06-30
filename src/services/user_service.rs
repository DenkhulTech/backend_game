use diesel::*;
use crate::models::user::{User, NewUser, UpdateUser};
use crate::schema::users::dsl::*;
use anyhow::Result;

pub struct UserService;

impl UserService {
    pub fn create_user(
        conn: &mut PgConnection,
        new_user: NewUser,
    ) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users)
            .values(&new_user)
            .returning(User::as_select())
            .get_result(conn)
    }

    pub fn get_all_users(
        conn: &mut PgConnection,
    ) -> Result<Vec<User>, diesel::result::Error> {
        let limit = 10;
        let offset = 0;
        users
            .select(User::as_returning())
            .limit(limit)
            .offset(offset)
            .load(conn)
    }

    pub fn get_user(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<Option<User>, diesel::result::Error> {
        users
            .find(user_id)
            .select(User::as_select())
            .first::<User>(conn)
            .optional()
    }

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

    pub fn delete_user(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(users.find(user_id))
            .execute(conn)
    }
}