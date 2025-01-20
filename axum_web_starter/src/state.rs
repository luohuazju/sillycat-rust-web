use crate::models::customer::Customer;
use crate::models::seller::Seller;
use std::collections::HashMap;
use uuid::Uuid;

pub struct AppState {
    pub customers: HashMap<Uuid, Customer>,
    pub sellers: HashMap<Uuid, Seller>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            customers: HashMap::new(),
            sellers: HashMap::new(),
        }
    }
}
