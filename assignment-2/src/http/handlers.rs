use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{issuance::CertificateIssuer, model::IssueCertificateCommand};

pub async fn health() -> StatusCode {
    // Liveness only proves the process can respond.
    StatusCode::OK
}

pub async fn ready() -> StatusCode {
    // A production readiness check would verify downstream dependencies such
    // as PostgreSQL and the certificate authority.
    StatusCode::OK
}

#[derive(Debug, Deserialize)]
pub struct IssueCertificateRequest {
    pub tenant_id: Uuid,
    pub csr_pem: String,
    pub requested_subject: String,
    pub requested_sans: Vec<String>,
    pub ttl_days: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct IssueCertificateResponse {
    pub certificate_id: Uuid,
    pub serial_number: String,
    pub certificate_pem: String,
    pub not_after: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct CertificateMetadataResponse {
    pub certificate_id: Uuid,
    pub serial_number: String,
    pub subject: String,
    pub issuer: String,
    pub san_entries: Vec<String>,
    pub not_before: chrono::DateTime<chrono::Utc>,
    pub not_after: chrono::DateTime<chrono::Utc>,
    pub certificate_pem: String,
}

impl From<crate::domain::model::IssuedCertificate> for CertificateMetadataResponse {
    fn from(certificate: crate::domain::model::IssuedCertificate) -> Self {
        Self {
            certificate_id: certificate.certificate_id,
            serial_number: certificate.serial_number,
            subject: certificate.subject,
            issuer: certificate.issuer,
            san_entries: certificate.san_entries,
            not_before: certificate.not_before,
            not_after: certificate.not_after,
            certificate_pem: certificate.certificate_pem,
        }
    }
}

pub async fn issue_certificate(
    State(issuer): State<Arc<CertificateIssuer>>,
    Json(request): Json<IssueCertificateRequest>,
) -> Result<(StatusCode, Json<IssueCertificateResponse>), StatusCode> {
    // Translate the wire request into the domain command shape before applying
    // issuance policy. This keeps DTO choices out of the domain module.
    let command = IssueCertificateCommand {
        tenant_id: request.tenant_id,
        // In production this comes from verified JWT claims or mesh identity,
        // not from the request body. The field is explicit here to show where
        // security context enters the domain workflow.
        actor: "assessment-user".to_string(),
        csr_pem: request.csr_pem,
        requested_subject: request.requested_subject,
        requested_sans: request.requested_sans,
        ttl_days: request.ttl_days.unwrap_or(30),
    };

    let issued = issuer
        .issue(command)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok((
        StatusCode::CREATED,
        Json(IssueCertificateResponse {
            certificate_id: issued.certificate_id,
            serial_number: issued.serial_number,
            certificate_pem: issued.certificate_pem,
            not_after: issued.not_after,
        }),
    ))
}

pub async fn list_certificates(
    State(issuer): State<Arc<CertificateIssuer>>,
) -> Result<Json<Vec<CertificateMetadataResponse>>, StatusCode> {
    let certificates = issuer
        .list_certificates()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(CertificateMetadataResponse::from)
        .collect();

    Ok(Json(certificates))
}

pub async fn get_certificate(
    State(issuer): State<Arc<CertificateIssuer>>,
    Path(certificate_id): Path<Uuid>,
) -> Result<Json<CertificateMetadataResponse>, StatusCode> {
    let certificate = issuer
        .get_certificate(certificate_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(CertificateMetadataResponse::from(certificate)))
}
