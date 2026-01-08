use std::net::{Ipv4Addr, SocketAddr};

use axum::{Router, http::StatusCode, routing::get};
use tokio::net::TcpListener;

/// Health check handler
async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Hello world handler
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[tokio::main]
async fn main() {
    // Router config
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/health", get(health_check));

    // Listen for requests on localhost:8080
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    // Start up a listener bound to the specified address
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);

    // Start up server
    axum::serve(listener, app).await.unwrap();
}

#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;

    assert_eq!(status_code, StatusCode::OK);
}
