// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    items (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Varchar,
        quantity -> Int4,
        category_id -> Uuid,
        price -> Int4,
        image_url -> Varchar,
        duration -> Nullable<Interval>,
        expiry_date -> Nullable<Date>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(items -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    items,
);
