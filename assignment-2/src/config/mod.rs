use std::env;

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Settings {
    pub bind_addr: String,
    pub database_url: String,
}

impl Settings {
    pub fn from_env() -> anyhow::Result<Self> {
        // Environment variables are used instead of checked-in config files so
        // the same image can be promoted through dev, staging, and production.
        // Secrets should be projected by Vault/External Secrets in production.
        Ok(Self {
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            database_url: env::var("DATABASE_URL").context("DATABASE_URL is required")?,
        })
    }
}
