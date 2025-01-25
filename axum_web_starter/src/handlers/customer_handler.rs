use axum::extract::{State, Json, Path};
use std::sync::Arc;
use axum::http::StatusCode;
use crate::daos::customer_dao::CustomerDAO;
use crate::models::customer::Customer;
use crate::state::AppState;
use uuid::Uuid;
use serde::Deserialize;

pub async fn create_customer(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CustomerPayload>,
) -> Result<Json<Customer>, (StatusCode, String)> {
    CustomerDAO::create_customer(&app_state.db_pool, payload.name, payload.email)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn list_customers(State(app_state): State<Arc<AppState>>) -> Result<Json<Vec<Customer>>, (StatusCode, String)> {
    CustomerDAO::list_customers(&app_state.db_pool)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn get_customer(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Customer>, (StatusCode, String)> {
    CustomerDAO::get_customer(&app_state.db_pool, id)
        .await
        .map(Json)
        .map_err(|_| (StatusCode::NOT_FOUND, "Customer not found".to_string()))
}

pub async fn update_customer(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CustomerPayload>,
) -> Result<Json<Customer>, (StatusCode, String)> {
    CustomerDAO::update_customer(&app_state.db_pool, id, payload.name, payload.email)
        .await
        .map(Json)
        .map_err(|_| (StatusCode::NOT_FOUND, "Customer not found".to_string()))
}

pub async fn delete_customer(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<&'static str, (StatusCode, String)> {
    let rows_affected = CustomerDAO::delete_customer(&app_state.db_pool, id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Customer not found".to_string()))?;

    if rows_affected > 0 {
        Ok("Customer deleted")
    } else {
        Err((StatusCode::NOT_FOUND, "Customer not found".to_string()))
    }
}

#[derive(Deserialize)]
pub struct CustomerPayload {
    pub name: String,
    pub email: String,
}
