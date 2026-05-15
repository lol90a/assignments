use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Command accepted by the domain layer when a tenant requests a certificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCertificateCommand {
    pub tenant_id: Uuid,
    pub actor: String,
    pub csr_pem: String,
    pub requested_subject: String,
    pub requested_sans: Vec<String>,
    pub ttl_days: i64,
}

/// Certificate material and metadata returned after successful issuance.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct IssuedCertificate {
    pub certificate_id: Uuid,
    pub serial_number: String,
    pub subject: String,
    pub issuer: String,
    pub san_entries: Vec<String>,
    pub not_before: DateTime<Utc>,
    pub not_after: DateTime<Utc>,
    pub certificate_pem: String,
}
