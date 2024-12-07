use axum::Router;
use axum::routing::{get, post};
use crate::api::handler::get_transactions;

pub fn create_router() -> Router {
    Router::new()
        .route("/transactions", get(get_transactions))  // 获取交易记录
}