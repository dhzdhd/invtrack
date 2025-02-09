use axum::{extract::State, http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;
use diesel::{RunQueryDsl, SelectableHelper};

use crate::{internal_server_error, models::item::{Item, NewItem}, schema::items::{self}};

pub async fn get_items() -> &'static str {
    "hi"
}

pub async fn create_item(
    State(pool): State<Pool>,
    Json(new_item): Json<NewItem>,
) -> Result<Json<Item>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_server_error)?;
    let res = conn
        .interact(|conn| {
            // let updated_item = Item::new_from_sub_item(1, new_item);

            diesel::insert_into(items::table)
                .values(new_item)
                .returning(Item::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_server_error)?
        .map_err(internal_server_error)?;
    Ok(Json(res))
}