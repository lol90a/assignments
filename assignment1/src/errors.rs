use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

/// Application-level errors that can be converted directly into HTTP responses.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Certificate not found")]
    NotFound,

    #[error("Database error")]
    Db(#[from] sqlx::Error),

    #[error("Failed to parse certificate: {0}")]
    ParseError(String),

    #[error("Internal server error")]
    Internal,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Keep client-facing messages stable while logging sensitive/internal
        // details, such as SQL errors, only on the server side.
        let (status, message) = match &self {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Db(err) => {
                tracing::error!(error = %err, "database operation failed");
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::ParseError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(ErrorResponse {
            code: status.as_u16(),
            message,
        });

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
