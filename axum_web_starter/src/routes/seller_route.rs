use axum::{
    extract::{State, Json, Path},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use serde::Deserialize;
use axum::http::StatusCode;

use crate::models::seller::Seller;
use crate::state::AppState;

pub fn seller_routes(app_state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/sellers", post(create_seller).get(list_sellers))
        .route("/sellers/{id}", get(get_seller).put(update_seller).delete(delete_seller))
        .with_state(app_state)
}

async fn create_seller(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Json(payload): Json<SellerPayload>,
) -> Json<Seller> {
    let mut state = app_state.write().await;
    let seller = Seller{
        id: Uuid::new_v4(),
        name: payload.name,
        company_name: payload.company_name,
    };
    state.sellers.insert(seller.id, seller.clone());
    Json(seller)
}

async fn list_sellers(State(app_state): State<Arc<RwLock<AppState>>>
    ) -> Json<Vec<Seller>>{
    let state = app_state.read().await;
    Json(state.sellers.values().cloned().collect())
}

async fn get_seller(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Seller>, (StatusCode, String)> {
    let state = app_state.read().await;
    if let Some(seller) = state.sellers.get(&id) {
        Ok(Json(seller.clone()))
    } else {
        Err((StatusCode::NOT_FOUND, "Seller not found".to_string()))
    }
}

async fn update_seller(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<SellerPayload>,
) -> Result<Json<Seller>, (StatusCode, String)> {
    let mut state = app_state.write().await;
    if let Some(seller) = state.sellers.get_mut(&id) {
        seller.name = payload.name;
        seller.company_name = payload.company_name;
        return Ok(Json(seller.clone()));
    }
    Err((StatusCode::NOT_FOUND, "Seller not found".to_string()))
}

async fn delete_seller(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<Uuid>,
) -> Result<&'static str, String> {
    let mut state = app_state.write().await;
    if state.sellers.remove(&id).is_some() {
        return Ok("Seller deleted");
    }
    Err("Seller not found".to_string())
}


#[derive(Deserialize)]
struct SellerPayload {
    name: String,
    company_name: String,
}