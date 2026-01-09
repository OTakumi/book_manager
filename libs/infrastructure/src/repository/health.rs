use async_trait::async_trait;
use derive_new::new;
use domain::repository::health::HealthCheckRepository;

use crate::database::ConnectionPool;

#[derive(new)]
pub struct HealthCheckRepositoryImple {
    db: ConnectionPool,
}

#[async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImple {
    async fn check_db(&self) -> bool {
        sqlx::query("SELECT 1")
            .fetch_one(self.db.inner_ref())
            .await
            .is_ok()
    }
}
