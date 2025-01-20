use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;

mod models;
mod routes;
mod state;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    // Shared state
    let app_state = Arc::new(RwLock::new(AppState::new()));

    // Define routes
    let app = Router::new()
        .merge(routes::customer_route::customer_routes(app_state.clone()))
        .merge(routes::seller_route::seller_routes(app_state.clone()))
        .route("/", get(|| async { "Axum CRUD API Starter!" }));

    println!("Server running on http://localhost:3000");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
