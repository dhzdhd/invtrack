use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::items;

#[derive(Default, Queryable, Selectable, Serialize)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    id: Uuid,
    name: String,
    description: String,
    quantity: i32,
    category_id: Uuid,
    price: i32,
    image_url: String,
    expiry_date: Option<NaiveDate>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = items)]
pub struct NewItem {
    name: String,
    description: String,
    quantity: i32,
    category_id: Uuid,
    price: i32,
    image_url: String,
    expiry_date: Option<NaiveDate>,
}
