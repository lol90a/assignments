use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;

/// Runtime settings loaded once during startup.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_addr: SocketAddr,
    pub log_filter: String,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        // Allow local `.env` files while still letting real environment
        // variables drive Docker and production-style deployments.
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
        // SERVER_ADDR is optional so local runs work without extra setup.
        let server_addr = env::var("SERVER_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:8080".to_string())
            .parse()?;
        let log_filter = env::var("RUST_LOG")
            .unwrap_or_else(|_| "certificate_service=info,tower_http=info".to_string());

        Ok(Self {
            database_url,
            server_addr,
            log_filter,
        })
    }
}
