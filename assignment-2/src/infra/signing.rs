use async_trait::async_trait;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::domain::{
    model::{IssueCertificateCommand, IssuedCertificate},
    ports::CertificateAuthority,
};

#[derive(Default)]
pub struct DummyCertificateAuthority;

#[async_trait]
impl CertificateAuthority for DummyCertificateAuthority {
    async fn sign_csr(
        &self,
        command: &IssueCertificateCommand,
    ) -> anyhow::Result<IssuedCertificate> {
        // Dummy issuance is intentionally behind the same trait a real CA would
        // implement. That prevents assessment scaffolding from infecting the
        // application architecture.
        let now = Utc::now();
        Ok(IssuedCertificate {
            certificate_id: Uuid::new_v4(),
            serial_number: Uuid::new_v4().to_string(),
            subject: command.requested_subject.clone(),
            issuer: "CN=Assessment Dummy CA".to_string(),
            san_entries: command.requested_sans.clone(),
            not_before: now,
            not_after: now + Duration::days(command.ttl_days),
            certificate_pem: "-----BEGIN CERTIFICATE-----\nDUMMY\n-----END CERTIFICATE-----"
                .to_string(),
        })
    }
}
