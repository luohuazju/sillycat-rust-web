use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::handlers::customer_handler::CustomerHandler;
use crate::state::AppState;

pub fn customer_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/customers", 
            post(CustomerHandler::create_customer)
            .get(CustomerHandler::list_customers))
        .route("/customers/{id}", 
            get(CustomerHandler::get_customer)
            .put(CustomerHandler::update_customer)
            .delete(CustomerHandler::delete_customer))
        .with_state(app_state)
}
