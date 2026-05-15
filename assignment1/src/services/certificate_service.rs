use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::Certificate;
use crate::models::dto::{
    CreateCertificateRequest, CreateCertificateResponse, ParseCertificateRequest,
    ParsedCertificateResponse,
};
use crate::repositories;
use chrono::DateTime;
use uuid::Uuid;
use x509_parser::extensions::GeneralName;
use x509_parser::pem::parse_x509_pem;
use x509_parser::prelude::{FromDer, X509Certificate};

pub async fn create_certificate(
    pool: &DbPool,
    request: CreateCertificateRequest,
) -> Result<CreateCertificateResponse, AppError> {
    // Validation lives in the service layer (not the handler) so business
    // rules can be reused by tests and future transports (gRPC, queues).
    request.validate()?;

    // The service constructs the domain model to keep persistence details
    // out of the HTTP layer and preserve separation of concerns.
    let certificate = Certificate {
        certificate_id: Uuid::new_v4(),
        subject: request.subject,
        issuer: request.issuer,
        expiration: request.expiration,
        san_entries: request.san_entries,
    };

    repositories::certificate_repository::insert_certificate(pool, &certificate).await?;

    Ok(CreateCertificateResponse {
        certificate_id: certificate.certificate_id,
    })
}

pub async fn list_certificates(pool: &DbPool) -> Result<Vec<Certificate>, AppError> {
    repositories::certificate_repository::list_certificates(pool).await
}

pub async fn get_certificate(pool: &DbPool, certificate_id: Uuid) -> Result<Certificate, AppError> {
    let certificate =
        repositories::certificate_repository::find_certificate_by_id(pool, certificate_id).await?;

    // The repository returns Option<Certificate>. Mapping None to a domain
    // not-found error here keeps HTTP semantics out of repository code.
    certificate.ok_or(AppError::NotFound)
}

pub fn parse_certificate(
    request: ParseCertificateRequest,
) -> Result<ParsedCertificateResponse, AppError> {
    let pem = request.certificate_pem.trim();
    let (_, pem_block) = parse_x509_pem(pem.as_bytes())
        .map_err(|error| AppError::ParseError(format!("failed to parse PEM block: {}", error)))?;

    // Parse PEM -> DER -> X.509 in explicit stages so failures are specific.
    let parsed = X509Certificate::from_der(&pem_block.contents)
        .map_err(|error| AppError::ParseError(format!("failed to decode x509 DER: {}", error)))?
        .1;

    let subject = parsed.subject().to_string();
    let issuer = parsed.issuer().to_string();
    let expiration_time = parsed.validity().not_after.to_datetime();
    let expiration = DateTime::from_timestamp(
        expiration_time.unix_timestamp(),
        expiration_time.nanosecond(),
    )
    .ok_or_else(|| {
        AppError::ParseError("certificate expiration is outside the supported date range".into())
    })?;
    let san_entries = extract_san_entries(&parsed);

    Ok(ParsedCertificateResponse {
        subject,
        issuer,
        expiration,
        san_entries,
    })
}

fn extract_san_entries(cert: &X509Certificate) -> Vec<String> {
    // We intentionally keep a conservative SAN extraction strategy. Only
    // frequently used identity-bearing SAN types are returned to callers.
    cert.subject_alternative_name()
        .ok()
        .flatten()
        .map(|san| {
            san.value
                .general_names
                .iter()
                .filter_map(|name| match name {
                    GeneralName::DNSName(name) => Some(name.to_string()),
                    GeneralName::RFC822Name(name) => Some(name.to_string()),
                    GeneralName::URI(name) => Some(name.to_string()),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default()
}
