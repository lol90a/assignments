use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::model::{IssueCertificateCommand, IssuedCertificate};

/// Persistence boundary for issued certificate metadata and audit evidence.
#[async_trait]
pub trait CertificateRepository: Send + Sync {
    async fn persist_issued_certificate(
        &self,
        command: &IssueCertificateCommand,
        certificate: &IssuedCertificate,
    ) -> anyhow::Result<()>;

    async fn list_certificates(&self) -> anyhow::Result<Vec<IssuedCertificate>>;

    async fn find_certificate_by_id(
        &self,
        certificate_id: Uuid,
    ) -> anyhow::Result<Option<IssuedCertificate>>;
}

/// Signing boundary so the domain does not depend on a specific CA product.
#[async_trait]
pub trait CertificateAuthority: Send + Sync {
    async fn sign_csr(
        &self,
        command: &IssueCertificateCommand,
    ) -> anyhow::Result<IssuedCertificate>;
}
