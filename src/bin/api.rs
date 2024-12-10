use hyper::header::HeaderValue;
use hyper::Method;
use my_ledger_backend::api::router::create_router;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let allowed_origins = vec![HeaderValue::from_static("http://localhost:5173")];
    
    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = create_router().layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
