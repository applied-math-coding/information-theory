use dotenv::dotenv;
use futures::stream::TryStreamExt;
use sqlx::{postgres::PgPoolOptions, Result, Row};
use std::env;
mod data_utils;

const DB_NAME: &str = "DB_NAME";
const DB_USER: &str = "DB_USER";
const DB_PWD: &str = "DB_PWD";
const DB_CONNECTIONS: &str = "DB_CONNECTIONS";

#[tokio::main]
async fn main() -> Result<()> {
    // data_utils::combine_data()?;
    dotenv().ok();
    let db_name: String = env::var(DB_NAME).unwrap();
    let db_user = env::var(DB_USER).unwrap();
    let db_pwd = env::var(DB_PWD).unwrap();
    let db_connections = std::env::var(DB_CONNECTIONS).unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(db_connections.parse().unwrap())
        .connect(&format!(
            "postgres://{}:{}@localhost/{}",
            db_user, db_pwd, db_name
        ))
        .await?;

    let mut rows =
        sqlx::query("SELECT sp.ticker FROM stock_prices sp GROUP BY sp.ticker").fetch(&pool);
    while let Some(ticker) = rows.try_next().await? {
        println!("{:?}", ticker.try_get::<String, usize>(0)?);
    }
    Ok(())
}
