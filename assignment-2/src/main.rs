mod config;
mod domain;
mod http;
mod infra;
mod observability;
mod security;

use std::sync::Arc;

use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::config::Settings;
use crate::domain::issuance::CertificateIssuer;
use crate::infra::{postgres::PostgresCertificateRepository, signing::DummyCertificateAuthority};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    observability::tracing::init_tracing();

    let settings = Settings::from_env()?;
    let pool = infra::postgres::connect(&settings.database_url).await?;

    // The application is assembled around traits rather than concrete adapters.
    // That keeps handlers stable when the signing backend moves from dummy
    // issuance to Vault PKI, AWS Private CA, or an HSM-backed internal CA.
    let repository = Arc::new(PostgresCertificateRepository::new(pool));
    let certificate_authority = Arc::new(DummyCertificateAuthority::default());
    let issuer = Arc::new(CertificateIssuer::new(repository, certificate_authority));

    let app: Router = http::router::build_router(issuer);
    let listener = TcpListener::bind(&settings.bind_addr)
        .await
        .with_context(|| format!("failed to bind {}", settings.bind_addr))?;

    info!(bind_addr = %settings.bind_addr, "certificate service listening");
    axum::serve(listener, app).await?;
    Ok(())
}
