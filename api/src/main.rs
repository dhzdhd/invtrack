use axum::{
    routing::get,
    Router,
};
use deadpool_diesel::postgres::{Manager, Pool, Runtime};
use diesel::prelude::*;
use lambda_http::{http::StatusCode, run, tracing, Error};
use routes::item::get_items;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;

mod routes;
mod tables;
mod models;

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
        published -> Bool,
    }
}

#[derive(Default, Queryable, Selectable, Serialize)]
struct Post {
    id: i32,
    title: String,
    content: String,
    published: bool,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)]
struct NewPost {
    title: String,
    content: String,
    published: bool,
}


async fn index() -> &'static str {
    "Hello World" 
}

type ServerError = (StatusCode, String);

fn internal_server_error<E: std::error::Error>(err: E) -> ServerError {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Required to enable CloudWatch error logging by the runtime
    tracing::init_default_subscriber();

    if cfg!(debug_assertions) {
        dotenv()?;
    }

    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL environment variable");
    let config = Manager::new(db_url, Runtime::Tokio1);
    let pool = Pool::builder(config).build().unwrap();

    let app = Router::new().route("/", get(index)).route("/items", get(get_items)).with_state(pool);

    if cfg!(debug_assertions) {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
        axum::serve(listener, app).await?;
    } else {
        run(app).await?;
    }

    Ok(())
}
