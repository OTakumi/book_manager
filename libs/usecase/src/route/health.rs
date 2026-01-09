use std::sync::Arc;

use axum::{Router, routing::get};

use crate::handler::health::{health_check, health_check_db};

use domain::repository::health::HealthCheckRepository;

pub async fn build_health_check_routers() -> Router<Arc<dyn HealthCheckRepository>> {
    // Health check router config
    let routers = Router::new()
        .route("/", get(health_check))
        .route("/db", get(health_check_db));

    Router::new().nest("/health", routers)
}
