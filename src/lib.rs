use data_utils::fetch_ticker_data_scatter;
use db_connections::create_db_connection;
use dotenv::dotenv;
use pyo3::prelude::*;
mod constants;
mod data_utils;
pub mod db_connections;
pub mod api;
mod repo;

#[pyfunction]
fn get_ticker_data_scatter(py: Python, ticker1: String, ticker2: String) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        dotenv().ok();
        let mut connection = create_db_connection().await.unwrap();
        let data = fetch_ticker_data_scatter(&mut connection, &ticker1, &ticker2)
            .await
            .unwrap();
        Ok(data)
    })
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
/// For this to work use 'crate-type = ["cdylib"]' in Cargo.toml
#[pymodule]
fn information_theory(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_ticker_data_scatter, m)?)?;
    Ok(())
}
