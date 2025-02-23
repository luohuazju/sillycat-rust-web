use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use dotenv::dotenv;
use utoipa::OpenApi;

mod models;
mod daos;
mod routes;
mod handlers;
mod state;
mod api_doc;

use crate::state::AppState;
use api_doc::ApiDoc;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::EnvFilter;


#[tokio::main]
async fn main() {
    dotenv().ok();
    // Get the DATABASE_URL from the environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");

    let redis_url = std::env::var("REDIS_URL")
        .expect("REDIS_URL must be set in the .env file");
    // Shared state
    let app_state = Arc::new(AppState::new(&database_url, &redis_url).await.unwrap());

    //init the logging
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::try_from_default_env() // Tries to read RUST_LOG
                .unwrap_or_else(|_| EnvFilter::new("info")), // Fallback to "info" if not set
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Define routes
    let app = Router::new()
        .merge(routes::customer_route::customer_routes(app_state.clone()))
        .merge(routes::seller_route::seller_routes(app_state.clone()))
        .route("/", get(|| async { "Axum CRUD API Starter!" }))
        .route("/openapi.json", get(openapi_json));

    println!("Server running on http://localhost:3000");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler for the OpenAPI JSON route
async fn openapi_json() -> impl axum::response::IntoResponse {
    let api_doc = ApiDoc::openapi(); // Generate OpenAPI JSON using utoipa
    axum::Json(api_doc) // Return as JSON
}
