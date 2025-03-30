use crate::{
    internal_server_error,
    models::item::{GeminiResponse, Item, NewItem},
    schema::items::{dsl::items, table},
};
use axum::{extract::State, http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;
use diesel::{RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct ImagePayload {
    image: String,
}

pub async fn create_item(
    State(pool): State<Pool>,
    Json(new_item): Json<NewItem>,
) -> Result<Json<Item>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_server_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(table)
                .values(new_item)
                .returning(Item::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_server_error)?
        .map_err(internal_server_error)?;
    Ok(Json(res))
}

pub async fn get_items(State(pool): State<Pool>) -> Result<Json<Vec<Item>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_server_error)?;
    let res = conn
        .interact(|conn| items.load(conn))
        .await
        .map_err(internal_server_error)?
        .map_err(internal_server_error)?;

    Ok(Json(res))
}

const GEMINI_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent";

pub async fn analyze_image(
    Json(payload): Json<ImagePayload>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let b64_str = payload.image;
    let gemini_key = std::env::var("GEMINI_API_KEY").map_err(internal_server_error)?;

    let client = reqwest::Client::new();
    let json = json!({
      "contents": [{
        "role": "user",
        "parts":[
            {"text": "List all items in the image."},
            {
              "inline_data": {
                "mime_type":"image/jpeg",
                "data": b64_str,
              }
            }
        ]
      }],
      "generationConfig": {
        "responseMimeType": "application/json",
      },
    });

    let res = client
        .post(GEMINI_URL)
        .header("Content-Type", "application/json")
        .query(&[("key", gemini_key)])
        .json(&json)
        .send()
        .await
        .map_err(internal_server_error)?;

    let resp = res
        .json::<GeminiResponse>()
        .await
        .map_err(internal_server_error)?;

    let names = resp
        .candidates
        .iter()
        .map(|candidate| candidate.content.parts.iter().map(|part| part.text.clone()))
        .flatten()
        .map(|str| serde_json::from_str::<Vec<String>>(str.as_str()).map_err(internal_server_error))
        .flatten()
        .flatten()
        .collect::<Vec<String>>();

    Ok(Json::from(names))
}
