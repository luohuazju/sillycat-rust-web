use axum::extract::{State, Json, Path};
use std::sync::Arc;
use axum::http::StatusCode;
use crate::daos::seller_dao::SellerDAO;
use crate::models::seller::Seller;
use crate::state::AppState;
use uuid::Uuid;
use serde::Deserialize;

pub struct SellerHandler;

impl SellerHandler {
    pub async fn create_seller(
        State(app_state): State<Arc<AppState>>,
        Json(payload): Json<SellerPayload>,
    ) -> Result<Json<Seller>, (StatusCode, String)> {
        SellerDAO::create_seller(&app_state.db_pool, payload.name, payload.company_name)
            .await
            .map(Json)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
    
    pub async fn list_sellers(
        State(app_state): State<Arc<AppState>>
    ) -> Result<Json<Vec<Seller>>, (StatusCode, String)>{
        SellerDAO::list_sellers(&app_state.db_pool)
            .await
            .map(Json)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
    
    pub async fn get_seller(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Seller>, (StatusCode, String)> {
        SellerDAO::get_seller(&app_state.db_pool, id)
            .await
            .map(Json)
            .map_err(|_| (StatusCode::NOT_FOUND, "Seller not found".to_string()))
    }
    
    pub async fn update_seller(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
        Json(payload): Json<SellerPayload>,
    ) -> Result<Json<Seller>, (StatusCode, String)> {
        SellerDAO::update_seller(&app_state.db_pool, id, payload.name, payload.company_name)
            .await
            .map(Json)
            .map_err(|_| (StatusCode::NOT_FOUND, "Seller not found".to_string()))
    }
    
    pub async fn delete_seller(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> Result<&'static str, (StatusCode, String)> {
        let rows_affected = SellerDAO::delete_seller(&app_state.db_pool, id)
            .await
            .map_err(|_| (StatusCode::NOT_FOUND, "Seller not found".to_string()))?;
    
        if rows_affected > 0 {
            Ok("Seller deleted")
        } else {
            Err((StatusCode::NOT_FOUND, "Seller not found".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct SellerPayload {
    name: String,
    company_name: String,
}