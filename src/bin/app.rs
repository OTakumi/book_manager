use anyhow::{Error, Result};
use axum::{Router, routing::get};
use config::config::AppConfig;
use infrastructure::{
    database::connect_database_with, repository::health::HealthCheckRepositoryImple,
};
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;
use usecase::route::health::build_health_check_routers;

/// Hello world handler
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    // AppConfig
    let app_config = AppConfig::new()?;

    // Connection Database
    let conn_pool = connect_database_with(&app_config.database);

    let health_check_repository = Arc::new(HealthCheckRepositoryImple::new(conn_pool.clone()));

    // Router config
    let app = Router::new()
        .route("/hello", get(hello_world))
        .merge(build_health_check_routers().await)
        .with_state(health_check_repository.clone());

    // Listen for requests on localhost:8080
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    // Start up a listener bound to the specified address
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);

    // Start up server
    axum::serve(listener, app).await.map_err(Error::from)
}
