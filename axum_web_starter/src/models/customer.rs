use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub email: String,    
}
