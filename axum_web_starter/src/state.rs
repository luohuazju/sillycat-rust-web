use sqlx::PgPool;
use std::sync::Arc;
// use opendal::services::Moka;
use opendal::services::Redis;
use tokio::time::Duration;
use opendal::Operator;
use anyhow::Result;


pub struct AppState {
    pub db_pool: PgPool,
    pub cache: Arc<Operator>, //OpenDAL Operator
}

impl AppState {
    pub async fn new(database_url: &str, redis_url: &str) -> Result<Self> {
        let db_pool = PgPool::connect(database_url).await?;
        
        // Initialize Moka as OpenDAL backend
        // let builder = Moka::default()
        //  .max_capacity(10_000)
        //  .time_to_live(Duration::from_secs(300));

        // Initialize Redis as OpenDAL backend
        let builder = Redis::default()
            .endpoint(redis_url)
            .default_ttl(Duration::from_secs(300));
        
        let op = Operator::new(builder)?.finish();
        
        Ok(Self { 
            db_pool,
            cache: Arc::new(op),
        })
    }
}
