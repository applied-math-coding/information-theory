use actix_web::{test, web, App};
use information_theory::{self, api::ticker_data::get_ticker_data};
mod common;

#[actix_web::test]
async fn test_get_ticker_data() {
    let app_state = common::setup().await.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .service(web::scope("/api").service(get_ticker_data)),
    )
    .await;
    let req = test::TestRequest::get()
        .uri("/api/ticker-data?ticker-name=^NSEI")
        .to_request();
    let resp: Vec<f32> = test::call_and_read_body_json(&app, req).await;
    assert!(resp.len() > 0);
}
