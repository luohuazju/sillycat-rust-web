use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub email: String,    
}
