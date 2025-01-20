use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Seller {
    pub id: Uuid,
    pub name: String,
    pub company_name: String,
}
