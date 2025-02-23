use sqlx::PgPool;
use std::sync::Arc;
use opendal::services::Moka;
use tokio::time::Duration;
use opendal::Operator;
use anyhow::Result;


pub struct AppState {
    pub db_pool: PgPool,
    pub cache: Arc<Operator>, //OpenDAL Operator
}

impl AppState {
    pub async fn new(database_url: &str) -> Result<Self> {
        let db_pool = PgPool::connect(database_url).await?;
        
        // Initialize Moka as OpenDAL backend
        let moka_builder = Moka::default()
        .max_capacity(10_000)
        .time_to_live(Duration::from_secs(300));


        let op = Operator::new(moka_builder)?.finish();
        
        Ok(Self { 
            db_pool,
            cache: Arc::new(op),
        })
    }
}
