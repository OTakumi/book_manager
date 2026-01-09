use axum::{extract::State, http::StatusCode};
use domain::repository::health::HealthCheckRepository;
use std::sync::Arc;

/// Health check handler
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Database health check handler
pub async fn health_check_db(
    State(health_check_repository): State<Arc<dyn HealthCheckRepository>>,
) -> StatusCode {
    let connection_result = health_check_repository.check_db().await;

    if connection_result {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
