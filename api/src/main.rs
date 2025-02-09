use axum::{
    routing::get, Router
};
use deadpool_diesel::postgres::{Manager, Pool, Runtime};
use lambda_http::{http::StatusCode, run, tracing, Error};
use routes::item::{create_item, get_items};
use dotenvy::dotenv;

mod routes;
mod models;
mod schema;

async fn index() -> &'static str {
    "Hello World" 
}

async fn test() -> &'static str {
    "Healthy" 
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

    let app = Router::new().route("/", get(index)).route("/health", get(test)).route("/items", get(get_items).post(create_item)).with_state(pool);

    if cfg!(debug_assertions) {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
        axum::serve(listener, app).await?;
    } else {
        run(app).await?;
    }

    Ok(())
}
