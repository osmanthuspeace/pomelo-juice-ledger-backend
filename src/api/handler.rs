use crate::service;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::{Datelike, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TransactionsQuery {
    from: String,
    to: String,
}
pub async fn get_transactions(
    Query(params): Query<TransactionsQuery>,
) -> impl IntoResponse {
    let from = NaiveDate::parse_from_str(&params.from, "%Y-%m-%d").unwrap_or_else(|_| {
        NaiveDate::from_ymd_opt(Utc::now().year(), Utc::now().month(), 1).unwrap()
    });
    let to = NaiveDate::parse_from_str(&params.to, "%Y-%m-%d").unwrap_or_else(|_| {
        NaiveDate::from_ymd_opt(Utc::now().year(), Utc::now().month(), Utc::now().day()).unwrap()
    });
    let result = service::get_service::get_transactions(from, to);
    match result {
        Ok(transactions) => Ok(Json(transactions)),
        Err(err) => {
            eprintln!("Error querying transactions: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
pub fn create_transactions(){
    todo!()
}
