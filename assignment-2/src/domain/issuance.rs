use std::sync::Arc;

use anyhow::{bail, Context};
use uuid::Uuid;

use crate::domain::{
    model::{IssueCertificateCommand, IssuedCertificate},
    ports::{CertificateAuthority, CertificateRepository},
};

/// Domain service that applies issuance policy before delegating to adapters.
pub struct CertificateIssuer {
    repository: Arc<dyn CertificateRepository>,
    certificate_authority: Arc<dyn CertificateAuthority>,
}

impl CertificateIssuer {
    pub fn new(
        repository: Arc<dyn CertificateRepository>,
        certificate_authority: Arc<dyn CertificateAuthority>,
    ) -> Self {
        Self {
            repository,
            certificate_authority,
        }
    }

    pub async fn issue(
        &self,
        command: IssueCertificateCommand,
    ) -> anyhow::Result<IssuedCertificate> {
        validate_policy(&command)?;

        // Signing and storage are kept in a service method instead of the HTTP
        // handler so the workflow can be tested without constructing Axum
        // requests. This also keeps the future async boundary clear: external
        // signing and database writes are awaited explicitly.
        let issued = self
            .certificate_authority
            .sign_csr(&command)
            .await
            .context("certificate authority failed to sign CSR")?;

        self.repository
            .persist_issued_certificate(&command, &issued)
            .await
            .context("failed to persist issued certificate")?;

        Ok(issued)
    }

    pub async fn list_certificates(&self) -> anyhow::Result<Vec<IssuedCertificate>> {
        self.repository
            .list_certificates()
            .await
            .context("failed to list issued certificates")
    }

    pub async fn get_certificate(
        &self,
        certificate_id: Uuid,
    ) -> anyhow::Result<Option<IssuedCertificate>> {
        self.repository
            .find_certificate_by_id(certificate_id)
            .await
            .context("failed to fetch issued certificate")
    }
}

fn validate_policy(command: &IssueCertificateCommand) -> anyhow::Result<()> {
    // Keep issuance guardrails close to the workflow so every caller gets the
    // same policy enforcement, regardless of transport.
    if command.csr_pem.trim().is_empty() {
        bail!("CSR is required");
    }
    if command.requested_subject.trim().is_empty() {
        bail!("subject is required");
    }
    if command.requested_sans.is_empty() {
        bail!("at least one SAN is required");
    }
    if command.ttl_days <= 0 || command.ttl_days > 397 {
        bail!("ttl_days must be between 1 and 397");
    }
    Ok(())
}
