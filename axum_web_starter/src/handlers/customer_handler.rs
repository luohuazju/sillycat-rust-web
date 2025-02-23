use axum::extract::{State, Json, Path};
use std::sync::Arc;
use axum::http::StatusCode;
use crate::daos::customer_dao::CustomerDAO;
use crate::models::customer::Customer;
use crate::models::customer::CustomerPayload;
use crate::state::AppState;
use uuid::Uuid;
use tracing::{error, info};


pub struct CustomerHandler;

#[utoipa::path(
    post,
    path = "/customers",
    request_body = CustomerPayload,
    responses(
        (status = 201, description = "Customer created successfully", body = Customer),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_customer_api(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CustomerPayload>,
) -> Result<Json<Customer>, (StatusCode, String)> {
    match CustomerDAO::create_customer(&app_state.db_pool, payload.name, payload.email).await {
        Ok(customer) => {
            // Cache the newly created customer
            let cache_key = format!("customer:{}", customer.id);
            if let Err(e) = app_state.cache.write(&cache_key, serde_json::to_string(&customer).unwrap()).await {
                error!("Cache write error: {}", e);
            }
            Ok(Json(customer))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}


#[utoipa::path(
    get,
    path = "/customers",
    responses(
        (status = 200, description = "List of all customers", body = [Customer]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_customers_api(State(app_state): State<Arc<AppState>>) -> Result<Json<Vec<Customer>>, (StatusCode, String)> {
    CustomerDAO::list_customers(&app_state.db_pool)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}


#[utoipa::path(
    get,
    path = "/customers/{id}",
    params(
        ("id" = String, description = "ID of the customer to retrieve", example = "d290f1ee-6c54-4b01-90e6-d701748f0851")
    ),
    responses(
        (status = 200, description = "Customer details", body = Customer),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn get_customer_api(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Customer>, (StatusCode, String)> {
    let cache_key = format!("customer:{}", id);
    match app_state.cache.read(&cache_key).await {
        Ok(cached_customer) => {
            if let Ok(customer) = serde_json::from_slice(&cached_customer.to_vec()) {
                info!("Cache hit with cahce_key = {}", cache_key);
                return Ok(Json(customer));
            } else {
                error!("Cache read parse error");
            }
        }
        Err(e) => info!("Cache miss with cache_key: {}, with {}", cache_key, e),
    }

    match CustomerDAO::get_customer(&app_state.db_pool, id).await {
        Ok(customer) => {
            if let Err(e) = app_state.cache.write(&cache_key, serde_json::to_string(&customer).unwrap()).await {
                error!("Cache write error: {}", e);
            }
            Ok(Json(customer))
        }
        Err(_) => Err((StatusCode::NOT_FOUND, "Customer not found".to_string())),
    }
}

#[utoipa::path(
    put,
    path = "/customers/{id}",
    request_body = CustomerPayload,
    params(
        ("id" = String, description = "ID of the customer to retrieve", example = "d290f1ee-6c54-4b01-90e6-d701748f0851")
    ),
    responses(
        (status = 200, description = "Updated customer details", body = Customer),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn update_customer_api(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CustomerPayload>,
) -> Result<Json<Customer>, (StatusCode, String)> {
    match CustomerDAO::update_customer(&app_state.db_pool, id, payload.name, payload.email).await {
        Ok(customer) => {
            // Update cache
            let cache_key = format!("customer:{}", id);
            if let Err(e) = app_state.cache.write(&cache_key, serde_json::to_string(&customer).unwrap()).await {
                error!("Cache write error: {}", e);
            }
            Ok(Json(customer))
        }
        Err(_) => Err((StatusCode::NOT_FOUND, "Customer not found".to_string())),
    }
}

#[utoipa::path(
    delete,
    path = "/customers/{id}",
    params(
        ("id" = String, description = "ID of the customer to retrieve", example = "d290f1ee-6c54-4b01-90e6-d701748f0851")
    ),
    responses(
        (status = 200, description = "Customer deleted successfully"),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn delete_customer_api(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<&'static str, (StatusCode, String)> {
    match CustomerDAO::delete_customer(&app_state.db_pool, id).await {
        Ok(rows_affected) if rows_affected > 0 => {
            // Invalidate cache after deletion
            let cache_key = format!("customer:{}", id);
            if let Err(e) = app_state.cache.delete(&cache_key).await {
                error!("Cache delete error: {}", e);
            }
            Ok("Customer deleted")
        }
        _ => Err((StatusCode::NOT_FOUND, "Customer not found".to_string())),
    }
}

impl CustomerHandler {
    pub async fn create_customer(
        state: State<Arc<AppState>>,
        payload: Json<CustomerPayload>,
    ) -> Result<Json<Customer>, (StatusCode, String)> {
        create_customer_api(state, payload).await
    }

    pub async fn list_customers(state: State<Arc<AppState>>) -> Result<Json<Vec<Customer>>, (StatusCode, String)> {
        list_customers_api(state).await
    }

    pub async fn get_customer(
        state: State<Arc<AppState>>,
        id: Path<Uuid>,
    ) -> Result<Json<Customer>, (StatusCode, String)> {
        get_customer_api(state, id).await
    }

    pub async fn update_customer(
        state: State<Arc<AppState>>,
        id: Path<Uuid>,
        payload: Json<CustomerPayload>,
    ) -> Result<Json<Customer>, (StatusCode, String)> {
        update_customer_api(state, id, payload).await
    }

    pub async fn delete_customer(
        state: State<Arc<AppState>>,
        id: Path<Uuid>,
    ) -> Result<&'static str, (StatusCode, String)> {
        delete_customer_api(state, id).await
    }

}
