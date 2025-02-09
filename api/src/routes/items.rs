use crate::{
    internal_server_error,
    models::item::{Item, NewItem},
    schema::items::dsl::items,
    schema::items::table,
};
use axum::{extract::State, http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;
use diesel::{RunQueryDsl, SelectableHelper};

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
