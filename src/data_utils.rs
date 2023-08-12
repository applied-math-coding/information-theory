use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Result, Write},
};

use futures::TryStreamExt;
use sqlx::{PgExecutor, Pool, Postgres, Row};

pub fn combine_data() -> Result<()> {
    let mut data_file_writer = BufWriter::new(File::create("data/data.csv")?);
    let header = "Ticker,Date,Open,High,Low,Close,Adj Close,Volume";
    data_file_writer.write_all(header.as_bytes())?;
    data_file_writer.write(b"\n")?;

    for year in 2008..2024 {
        let file_name = format!("data/{}_Global_Markets_Data.csv", year);
        let reader = BufReader::new(File::open(file_name)?);
        let mut line_iter = reader.lines().skip(1);
        while let Some(Ok(line)) = line_iter.next() {
            data_file_writer.write_all(line.as_bytes())?;
            data_file_writer.write(b"\n")?;
        }
        data_file_writer.flush()?;
    }
    Ok(())
}

pub async fn fetch_ticker_data_scatter(
    executer: impl PgExecutor<'_>,
    ticker1: &str,
    ticker2: &str,
) -> sqlx::Result<(Vec<f32>, Vec<f32>)> {
    let rows = sqlx::query(
        r#"
    SELECT
      sp1.close_normalized,
      sp2.close_normalized
    FROM stock_prices sp1, stock_prices sp2
    WHERE sp1."date" = sp2."date"
      AND sp1.ticker = $1
      AND sp2.ticker = $2
    ORDER BY sp1."date" ASC
  "#,
    )
    .bind(ticker1)
    .bind(ticker2)
    .fetch_all(executer)
    .await?;
    let (mut ticker_vec_1, mut ticker_vec_2) = (vec![], vec![]);
    rows.iter().for_each(|row| {
        ticker_vec_1.push(row.get::<f32, usize>(0));
        ticker_vec_2.push(row.get::<f32, usize>(1));
    });
    Ok((ticker_vec_1, ticker_vec_2))
}

pub async fn normalize_data(pool: &Pool<Postgres>) -> sqlx::Result<()> {
    let mut rows =
        sqlx::query("SELECT sp.ticker FROM stock_prices sp GROUP BY sp.ticker").fetch(pool);
    while let Some(ticker) = rows.try_next().await? {
        let ticker_name = ticker.try_get::<String, usize>(0)?;
        let mut txn = pool.begin().await?;
        sqlx::query(
            r#"
          WITH "ranges" AS (
            SELECT MAX(sp."close") - MIN(sp."close") AS "range",
              MIN(sp."close") AS min_close
            FROM stock_prices sp
            WHERE sp.ticker = $1
          )
          UPDATE stock_prices AS sp SET ("close_normalized") =
              (SELECT (sp."close" - r.min_close) / r."range" FROM ranges r)
          WHERE sp.ticker = $1"#,
        )
        .bind(ticker_name)
        .execute(&mut txn)
        .await?;
        txn.commit().await?;
    }
    Ok(())
}
