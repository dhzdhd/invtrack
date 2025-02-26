use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use deadpool_diesel::postgres::{Manager, Pool, Runtime};
use dotenvy::dotenv;
use lambda_http::{http::StatusCode, run, tracing, Error};
use routes::{
    categories::{create_category, get_categories},
    items::{analyze_image, create_item, get_items},
};

mod models;
mod routes;
mod schema;

async fn index() -> &'static str {
    "Hello World"
}

async fn healthcheck() -> &'static str {
    "Healthy"
}

type ServerError = (StatusCode, String);

fn internal_server_error<E: std::error::Error>(err: E) -> ServerError {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    if cfg!(debug_assertions) {
        dotenv()?;
    }

    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL environment variable");
    let config = Manager::new(db_url, Runtime::Tokio1);
    let pool = Pool::builder(config).build().unwrap();

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(healthcheck))
        .route("/items", get(get_items).post(create_item))
        .route("/analyze", post(analyze_image))
        .route("/categories", get(get_categories).post(create_category))
        .layer(DefaultBodyLimit::max(51200)) // 50 MB
        .with_state(pool);

    if cfg!(debug_assertions) {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
        println!("Listening on http://localhost:8080");
        axum::serve(listener, app).await?;
    } else {
        run(app).await?;
    }

    Ok(())
}
