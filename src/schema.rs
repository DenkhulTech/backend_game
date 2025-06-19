diesel::table! {
    crops (id) {
        id -> Int4,
        name -> Text,
        grow_time -> Int4,
        yield_item -> Int4,
        sell_price -> Int4,
    }
}

diesel::table! {
    fields (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        tile_x -> Int4,
        tile_y -> Int4,
        is_tilled -> Bool,
        is_watered -> Bool,
        crop_id -> Nullable<Int4>,
        plant_time -> Nullable<Timestamp>,
        ready_time -> Nullable<Timestamp>,
        status -> Text,
    }
}

diesel::table! {
    inventory_items (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        item_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    tools (item_id) {
        item_id -> Int4,
        action -> Text,
        durability -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        wallet -> Nullable<Text>,
        session_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(fields -> users (user_id));
diesel::joinable!(inventory_items -> users (user_id));
diesel::joinable!(tools -> items (item_id));

diesel::allow_tables_to_appear_in_same_query!(
    crops,
    fields,
    inventory_items,
    items,
    tools,
    users,
);
