use axum::{Json, extract::Path, extract::State};
use serde::Serialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::errors::{AppError, AppResult};
use crate::models::dto::{CreateCertificateRequest, ParseCertificateRequest};
use crate::services::certificate_service;

#[derive(Debug, Serialize)]
pub struct CreatedResponse {
    pub certificate_id: Uuid,
}

pub async fn list_certificates(
    State(pool): State<DbPool>,
) -> AppResult<Json<Vec<crate::models::Certificate>>> {
    let certificates = certificate_service::list_certificates(&pool).await?;
    Ok(Json(certificates))
}

pub async fn create_certificate(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateCertificateRequest>,
) -> AppResult<Json<CreatedResponse>> {
    // Handlers stay thin: extract transport data, call service, map result.
    let response = certificate_service::create_certificate(&pool, payload).await?;

    Ok(Json(CreatedResponse {
        certificate_id: response.certificate_id,
    }))
}

pub async fn get_certificate(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AppResult<Json<crate::models::Certificate>> {
    // Parse and validate path params at the edge so domain/service layers
    // only receive strongly typed identifiers.
    let certificate_id =
        Uuid::parse_str(&id).map_err(|_| AppError::Validation("invalid certificate id".into()))?;
    let certificate = certificate_service::get_certificate(&pool, certificate_id).await?;

    Ok(Json(certificate))
}

pub async fn parse_certificate(
    Json(payload): Json<ParseCertificateRequest>,
) -> AppResult<Json<crate::models::dto::ParsedCertificateResponse>> {
    let parsed = certificate_service::parse_certificate(payload)?;
    Ok(Json(parsed))
}
