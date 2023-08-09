use std::env;

use sqlx::{postgres::PgPoolOptions, Connection, Error, PgConnection, Pool, Postgres};

use crate::constants::{DB_CONNECTIONS, DB_URL};

pub async fn create_db_pool() -> Result<Pool<Postgres>, Error> {
    let db_connections = env::var(DB_CONNECTIONS).unwrap();
    PgPoolOptions::new()
        .max_connections(db_connections.parse().unwrap())
        .connect(&env::var(DB_URL).unwrap())
        .await
}

pub async fn create_db_connection() -> Result<PgConnection, Error> {
    PgConnection::connect(&env::var(DB_URL).unwrap()).await
}
