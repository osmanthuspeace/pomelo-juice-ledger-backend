use crate::db::models::NewTransaction;
use crate::service;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{Datelike, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetQuery {
    pub from: Option<String>,
    pub to: Option<String>,
}
pub async fn get_transactions(Query(params): Query<GetQuery>) -> impl IntoResponse {
    let from = NaiveDate::parse_from_str(
        &params.from.unwrap_or(
            NaiveDate::from_ymd_opt(Utc::now().year(), Utc::now().month(), 1)
                .unwrap()
                .to_string(),
        ),
        "%Y-%m-%d",
    )
    .unwrap();
    let to = NaiveDate::parse_from_str(
        &params.to.unwrap_or(
            NaiveDate::from_ymd_opt(Utc::now().year(), Utc::now().month(), Utc::now().day())
                .unwrap()
                .to_string(),
        ),
        "%Y-%m-%d",
    )
    .unwrap();
    let result = service::get_service::get_transactions(from, to);
    match result {
        Ok(transactions) => Ok(Json(transactions)),
        Err(err) => {
            eprintln!("Error querying transactions: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct CreateQuery {
    pub date: String,
    pub kind: String,
    pub description: String,
    pub amount: f64,
    pub account: String,
}
pub async fn create_transactions(Query(params): Query<CreateQuery>) {
    let new_transaction = NewTransaction::new(
        NaiveDate::parse_from_str(&params.date, "%Y-%m-%d").unwrap(),
        params.kind,
        params.description,
        params.amount,
        params.account,
    );
    service::create_service::create_transaction(&new_transaction)
        .expect("Error creating transaction");
}
