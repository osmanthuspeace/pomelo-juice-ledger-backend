use axum::Router;
use axum::routing::{get, post};
use crate::api::handler::{create_transactions, get_transactions};

pub fn create_router() -> Router {
    Router::new()
        .route("/get", get(get_transactions))  
        .route("/create",post(create_transactions))  
}