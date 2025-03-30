use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::items;

#[derive(Deserialize, Debug)]

pub struct GeminiContentPart {
    pub text: String,
}
#[derive(Deserialize, Debug)]

pub struct GeminiContent {
    pub parts: Vec<GeminiContentPart>,
    pub role: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]

pub struct GeminiCandidate {
    pub avg_logprobs: f64,
    pub content: GeminiContent,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct GeminiResponse {
    pub candidates: Vec<GeminiCandidate>,
    pub model_version: String,
}

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
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewItem {
    name: String,
    description: String,
    quantity: i32,
    category_id: Uuid,
    price: i32,
    image_url: String,
    expiry_date: Option<NaiveDate>,
}
