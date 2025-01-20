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

use crate::models::customer::Customer;
use crate::state::AppState;

pub fn customer_routes(app_state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/customers", post(create_customer).get(list_customers))
        .route("/customers/{id}", get(get_customer).put(update_customer).delete(delete_customer))
        .with_state(app_state)
}

async fn create_customer(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Json(payload): Json<CustomerPayload>,
) -> Json<Customer> {
    let mut state = app_state.write().await;
    let customer = Customer{
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email,
    };
    state.customers.insert(customer.id, customer.clone());
    Json(customer)
}

async fn list_customers(State(app_state): State<Arc<RwLock<AppState>>>
    ) -> Json<Vec<Customer>>{
    let state = app_state.read().await;
    Json(state.customers.values().cloned().collect())
}

async fn get_customer(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Customer>, (StatusCode, String)> {
    let state = app_state.read().await;
    if let Some(customer) = state.customers.get(&id) {
        Ok(Json(customer.clone()))
    } else {
        Err((StatusCode::NOT_FOUND, "Customer not found".to_string()))
    }
}

async fn update_customer(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CustomerPayload>,
) -> Result<Json<Customer>, (StatusCode, String)> {
    let mut state = app_state.write().await;
    if let Some(customer) = state.customers.get_mut(&id) {
        customer.name = payload.name;
        customer.email = payload.email;
        return Ok(Json(customer.clone()));
    }
    Err((StatusCode::NOT_FOUND, "Customer not found".to_string()))
}

async fn delete_customer(
    State(app_state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<Uuid>,
) -> Result<&'static str, String> {
    let mut state = app_state.write().await;
    if state.customers.remove(&id).is_some() {
        return Ok("Customer deleted");
    }
    Err("Customer not found".to_string())
}


#[derive(Deserialize)]
struct CustomerPayload {
    name: String,
    email: String,
}