use sqlx::PgPool;
use moka::future::Cache;
use tokio::time::Duration;
use std::sync::Arc;


pub struct AppState {
    pub db_pool: PgPool,
    pub cache: Arc<Cache<String, String>>, //moka cache
}

impl AppState {
    pub async fn new(database_url: &str) -> Self {
        let db_pool = PgPool::connect(database_url).await.expect("Failed to connect to database");
        
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(300)) // 5 minutes TTL
            .max_capacity(10_000)
            .build();
        
        Self { 
            db_pool,
            cache: Arc::new(cache), 
        }
    }
}
