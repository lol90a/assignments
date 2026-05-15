use crate::errors::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCertificateRequest {
    pub subject: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub san_entries: Vec<String>,
}

impl CreateCertificateRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        // These are API contract checks, not database checks. Rejecting invalid
        // payloads here produces clear 400 responses before persistence.
        if self.subject.trim().is_empty() {
            return Err(AppError::Validation("subject is required".into()));
        }
        if self.issuer.trim().is_empty() {
            return Err(AppError::Validation("issuer is required".into()));
        }
        if self.expiration <= Utc::now() {
            return Err(AppError::Validation(
                "expiration must be in the future".into(),
            ));
        }
        if self.san_entries.is_empty() {
            return Err(AppError::Validation(
                "san_entries must include at least one value".into(),
            ));
        }
        if self.san_entries.iter().any(|entry| entry.trim().is_empty()) {
            return Err(AppError::Validation(
                "san_entries may not contain empty values".into(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct CreateCertificateResponse {
    pub certificate_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct ParseCertificateRequest {
    /// PEM-encoded certificate supplied by the caller for metadata extraction.
    pub certificate_pem: String,
}

#[derive(Debug, Serialize)]
pub struct ParsedCertificateResponse {
    pub subject: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub san_entries: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn validate_rejects_empty_subject() {
        let request = CreateCertificateRequest {
            subject: String::new(),
            issuer: "Issuer".into(),
            expiration: Utc::now() + Duration::days(10),
            san_entries: vec!["example.com".into()],
        };

        let result = request.validate();

        assert!(result.is_err());
        matches!(result.unwrap_err(), AppError::Validation(_));
    }

    #[test]
    fn validate_accepts_valid_request() {
        let request = CreateCertificateRequest {
            subject: "CN=example.com".into(),
            issuer: "CN=Test CA".into(),
            expiration: Utc::now() + Duration::days(30),
            san_entries: vec!["example.com".into()],
        };

        assert!(request.validate().is_ok());
    }
}
