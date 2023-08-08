use constants::DB_CONNECTIONS;
use data_utils::normalize_data;
use db_connections::create_db_pool;
use dotenv::dotenv;
use futures::{Stream, TryStreamExt};
use sqlx::{Result, Row};
mod constants;
mod data_utils;
mod db_connections;

#[tokio::main]
async fn main() -> Result<()> {
    // data_utils::combine_data()?;
    dotenv().ok();
    let pool = create_db_pool().await?;
    sqlx::migrate!("db/migrations").run(&pool).await?;
    // normalize_data(&pool).await?;
    Ok(())
}
