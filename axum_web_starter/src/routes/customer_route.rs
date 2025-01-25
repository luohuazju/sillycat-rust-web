use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::handlers::customer_handler;
use crate::state::AppState;

pub fn customer_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/customers", post(customer_handler::create_customer).get(customer_handler::list_customers))
        .route("/customers/{id}", get(customer_handler::get_customer).put(customer_handler::update_customer).delete(customer_handler::delete_customer))
        .with_state(app_state)
}
