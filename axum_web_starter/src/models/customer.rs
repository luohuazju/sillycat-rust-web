use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone, ToSchema)]
pub struct Customer {
    #[schema(value_type = String, example = "d290f1ee-6c54-4b01-90e6-d701748f0851")]
    pub id: Uuid,
    pub name: String,
    pub email: String,    
}

#[derive(Deserialize, ToSchema)]
pub struct CustomerPayload {
    pub name: String,
    pub email: String,
}
