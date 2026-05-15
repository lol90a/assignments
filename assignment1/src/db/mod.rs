use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

use crate::config::AppConfig;

pub type DbPool = PgPool;

pub async fn create_pool(config: &AppConfig) -> Result<DbPool, sqlx::Error> {
    // Pool limits keep the service from opening an unbounded number of
    // PostgreSQL connections under concurrent traffic.
    PgPoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_secs(300))
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await
}
