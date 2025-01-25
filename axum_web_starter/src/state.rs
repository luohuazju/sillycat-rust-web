use sqlx::PgPool;

pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub async fn new(database_url: &str) -> Self {
        let db_pool = PgPool::connect(database_url).await.expect("Failed to connect to database");
        Self { db_pool }
    }
}
