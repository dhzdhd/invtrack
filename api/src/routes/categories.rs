use crate::{
    internal_server_error,
    models::category::{Category, NewCategory},
    schema::categories::dsl::categories,
    schema::categories::table,
};
use axum::{extract::State, http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;
use diesel::{RunQueryDsl, SelectableHelper};

pub async fn create_category(
    State(pool): State<Pool>,
    Json(new_category): Json<NewCategory>,
) -> Result<Json<Category>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_server_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(table)
                .values(new_category)
                .returning(Category::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_server_error)?
        .map_err(internal_server_error)?;
    Ok(Json(res))
}

pub async fn get_categories(
    State(pool): State<Pool>,
) -> Result<Json<Vec<Category>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_server_error)?;
    let res = conn
        .interact(|conn| categories.load(conn))
        .await
        .map_err(internal_server_error)?
        .map_err(internal_server_error)?;

    Ok(Json(res))
}
