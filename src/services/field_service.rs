use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, OptionalExtension};
use crate::models::field::{Field, NewField, UpdateField};
use crate::schema::fields::dsl::*;
use anyhow::Result;

pub struct FieldService;

impl FieldService {
    // Create Field
    pub fn create_field(
        conn: &mut PgConnection,
        new_field: NewField,
    ) -> Result<Field, diesel::result::Error> {
        diesel::insert_into(fields)
            .values(&new_field)
            .returning(Field::as_select())
            .get_result(conn)
    }

    // Get All Fields
    pub fn get_all_fields(
        conn: &mut PgConnection,
    ) -> Result<Vec<Field>, diesel::result::Error> {
        fields
            .select(Field::as_returning())
            .load(conn)
    }

    // Get Field by ID
    pub fn get_field(
        conn: &mut PgConnection,
        field_id: i32,
    ) -> Result<Option<Field>, diesel::result::Error> {
        fields
            .find(field_id)
            .select(Field::as_select())
            .first(conn)
            .optional()
    }

    // Update Field
    pub fn update_field(
        conn: &mut PgConnection,
        field_id: i32,
        update_data: UpdateField,
    ) -> Result<Field, diesel::result::Error> {
        diesel::update(fields.find(field_id))
            .set(&update_data)
            .returning(Field::as_select())
            .get_result(conn)
    }

    // Delete Field
    pub fn delete_field(
        conn: &mut PgConnection,
        field_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(fields.find(field_id))
            .execute(conn)
    }

    // Get Fields by User ID
    pub fn get_fields_by_user(
        conn: &mut PgConnection,
        uuid: i32,
    ) -> Result<Vec<Field>, diesel::result::Error> {
        fields
            .filter(user_id.eq(uuid))
            .select(Field::as_select())
            .load(conn)
    }
}