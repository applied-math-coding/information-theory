use dotenv::dotenv;
use information_theory::{api::AppState, db_connections::create_db_pool};

// this could set a different DB for testing
pub async fn setup() -> Result<AppState, String> {
    dotenv().ok();
    env_logger::init();
    let pool = create_db_pool().await.map_err(|e| format!("{e}"))?;
    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await
        .map_err(|e| format!("{e}"))?;
    Ok(AppState { pool })
}
