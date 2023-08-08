use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Result, Write},
};

use futures::TryStreamExt;
use sqlx::{Pool, Postgres, Row};

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

pub async fn normalize_data(pool: &Pool<Postgres>) -> sqlx::Result<()> {
    let mut rows =
        sqlx::query("SELECT sp.ticker FROM stock_prices sp GROUP BY sp.ticker").fetch(pool);
    while let Some(ticker) = rows.try_next().await? {
        let ticker_name = ticker.try_get::<String, usize>(0)?;
        let mut txn = pool.begin().await?;
        sqlx::query(r#"
        with factor as (
          select max(sp."close") - min(sp."close") as close_factor
          from stock_prices sp
          where sp.ticker = $1
        )
        update stock_prices as sp set "close_normalized" = sp."close" / (select f.close_factor from factor f)
        where sp.ticker = $1"#)
        .bind(ticker_name)
        .execute(&mut txn).await?;
        txn.commit().await?;
    }
    Ok(())
}
