use sqlx::{PgExecutor, Row};

pub async fn find_all_ticker_data(
    executor: impl PgExecutor<'_>,
    ticker: &str,
) -> sqlx::Result<Vec<f32>> {
    let rows = sqlx::query(
        r#"
        SELECT
          sp.close
        FROM stock_prices sp
        WHERE sp.ticker = $1
        ORDER BY sp."date" ASC
    "#,
    )
    .bind(ticker)
    .fetch_all(executor)
    .await?;
    Ok(rows.iter().map(|row| row.get::<f32, usize>(0)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::AppState;

    #[actix_web::test]
    async fn test_find_all_ticker_data() {
        let AppState { pool } = AppState::init().await.unwrap();
        let data = find_all_ticker_data(&pool, "^NSEI").await.unwrap();
        assert!(data.len() > 0);
    }
}
