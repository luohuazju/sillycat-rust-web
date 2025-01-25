use axum::{
    routing::{get, post},
    Router,
};
use crate::handlers::seller_handler::SellerHandler;
use std::sync::Arc;
use crate::state::AppState;

pub fn seller_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/sellers", 
            post(SellerHandler::create_seller)
            .get(SellerHandler::list_sellers))
        .route("/sellers/{id}", 
            get(SellerHandler::get_seller)
            .put(SellerHandler::update_seller)
            .delete(SellerHandler::delete_seller))
        .with_state(app_state)
}
