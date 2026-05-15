use axum::Router;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use certificate_service::config::AppConfig;
use certificate_service::db::create_pool;
use certificate_service::routes::router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load config first so every later subsystem (logging, DB, network)
    // uses one consistent source of truth for environment-specific values.
    let config = AppConfig::load()?;

    // Tracing is initialized before any real work starts so startup failures
    // are observable in the same structured log pipeline as request logs.
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.log_filter))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!(server_addr = %config.server_addr, "starting certificate service");

    // Pool creation is separated from handlers so we can centralize connection
    // limits/timeouts and prevent each request path from creating new clients.
    let pool = create_pool(&config).await?;
    tracing::info!(database_url = %config.database_url, "database pool initialized");

    // Running migrations at startup keeps local and containerized environments
    // deterministic. In larger production setups this often moves to a
    // dedicated migration job to avoid concurrent migration races.
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("database migrations applied");

    let app: Router = router(pool);

    let listener = TcpListener::bind(config.server_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
