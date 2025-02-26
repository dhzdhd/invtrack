use crate::{
    internal_server_error,
    models::item::{Item, NewItem},
    schema::items::dsl::items,
    schema::items::table,
};
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use deadpool_diesel::postgres::Pool;
use diesel::{RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};

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
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent";

pub async fn analyze_image(
    Json(payload): Json<ImagePayload>,
) -> Result<Json<Option<Item>>, (StatusCode, String)> {
    // let mut b64_str = String::new();
    let b64_str = payload.image;
    let gemini_key = std::env::var("GEMINI_API_KEY").map_err(internal_server_error)?;

    // while let Ok(Some(field)) = image.next_field().await {
    //     println!("hefefee");

    //     // if let Ok(bytes) = field.bytes().await {
    //     let bytes = field.bytes().await.map_err(internal_server_error)?;
    //     println!("{}", bytes.len());
    //     // Disable line wrapping
    //     let base64 = BASE64_STANDARD.encode(bytes);
    //     b64_str.push_str(base64.as_str());
    //     // }
    // }

    let client = reqwest::Client::new();
    let json = json!({
      "contents": [{
        "parts":[
            {"text": "Caption this image."},
            {
              "inline_data": {
                "mime_type":"image/png",
                "data": b64_str,
              }
            }
        ]
      }]
    });
    println!("{json:?}");
    // let body = to_string(&json).map_err(internal_server_error)?;

    let res = client
        .post(GEMINI_URL)
        .header("Content-Type", "application/json")
        .query(&[("key", gemini_key)])
        .json(&json)
        .send()
        .await
        .map_err(internal_server_error)?;
    let resp: Value = res.json().await.map_err(internal_server_error)?;
    println!("{resp:?}");

    Ok(Json(None))
}
