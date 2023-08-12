use dotenv::dotenv;
use sqlx::{Pool, Postgres};

use crate::db_connections::create_db_pool;

pub mod ticker_data;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

impl AppState {
    pub async fn init() -> Result<AppState, String> {
        dotenv().ok();
        env_logger::init();
        let pool = create_db_pool().await.map_err(|e| format!("{e}"))?;
        sqlx::migrate!("db/migrations")
            .run(&pool)
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(AppState { pool })
    }
}
