use axum::Router;
use axum::routing::{get, post};
use crate::api::handler::{create_transactions, get_transactions};

pub fn create_router() -> Router {
    Router::new()
        .route("/get", get(get_transactions))  // 获取交易记录
        .route("/create",post(create_transactions))  // 获取所有交易记录
}