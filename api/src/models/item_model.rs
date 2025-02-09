use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::items;


#[derive(Default, Queryable, Selectable, Serialize)]
pub struct Item {
    id: i32,
    title: String,
}

impl Item {
    pub fn new_from_sub_item(id: i32, partial_item: NewItem) -> Self {
        Item {
            id,
            title: partial_item.title
        }
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = items)]
pub struct NewItem {
    title: String,
}