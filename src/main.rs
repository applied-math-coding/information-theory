use actix_web::{web, App, HttpServer};
use api::{ticker_data::get_ticker_data, AppState};
mod api;
mod constants;
mod data_utils;
mod db_connections;
mod repo;

#[tokio::main]
async fn main() -> Result<(), String> {
    // data_utils::combine_data()?;
    let app_state = AppState::init().await?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(web::scope("/api").service(get_ticker_data))
    })
    .bind(("127.0.0.1", 8080))
    .map_err(|e| format!("{e}"))?
    .run()
    .await
    .map_err(|e| format!("{e}"))?;
    Ok(())
}
