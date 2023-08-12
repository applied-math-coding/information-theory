use actix_web::{error, get, web, Responder, Result};
use log::info;
use serde::Deserialize;
use super::AppState;

use crate::repo::ticker_data_repo::find_all_ticker_data;

#[derive(Deserialize)]
pub struct GetTickerDataParams {
    #[serde(rename = "ticker-name")]
    ticker_name: String,
}

#[get("/ticker-data")]
pub async fn get_ticker_data(
    params: web::Query<GetTickerDataParams>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    info!("Fetching data for ticker {}", params.ticker_name);
    let ticker_data = find_all_ticker_data(&data.pool, &params.ticker_name)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(web::Json(ticker_data))
}
