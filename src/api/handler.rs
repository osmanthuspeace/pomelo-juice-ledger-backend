use crate::db::models::NewTransaction;
use crate::service;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetQuery {
    pub from: Option<String>,
    pub to: Option<String>,
}
pub async fn get_transactions(Query(params): Query<GetQuery>) -> impl IntoResponse {
    let (default_from, default_to) =
        service::get_service::get_default_date().unwrap_or_else(|_| {
            (
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2970, 1, 1).unwrap(),
            )
        });

    let from = match &params.from {
        Some(s) if s.is_empty() => default_from,
        None => default_from,
        Some(date_str) => match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(f) => f,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        },
    };
    let to = match &params.to {
        Some(s) if s.is_empty() => default_to,
        None => default_to,
        Some(date_str) => match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(t) => t,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        },
    };
    if from > to {
        return Err(StatusCode::BAD_REQUEST);
    }
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
#[derive(Serialize)]
struct ResponseMessage {
    message: String,
    transaction_id: Option<i32>,
}
pub async fn create_transactions(Json(params): Json<CreateQuery>) -> impl IntoResponse {
    let date = match NaiveDate::parse_from_str(&params.date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            let error_response = ResponseMessage {
                message: "日期格式应为 YYYY-MM-DD".to_string(),
                transaction_id: None,
            };
            return (StatusCode::BAD_REQUEST, Json(error_response));
        }
    };
    let new_transaction = NewTransaction::new(
        date,
        params.kind,
        params.description,
        params.amount,
        params.account,
    );
    match service::create_service::create_transaction(&new_transaction) {
        Ok(tx) => {
            let success_response = ResponseMessage {
                message: "交易记录创建成功".to_string(),
                transaction_id: Some(tx.id),
            };
            (StatusCode::CREATED, Json(success_response))
        }
        Err(e) => {
            eprintln!("创建交易记录失败: {}", e);
            let error_response = ResponseMessage {
                message: "创建交易记录失败".to_string(),
                transaction_id: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}
