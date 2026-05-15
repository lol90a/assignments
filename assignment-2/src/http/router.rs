use std::{sync::Arc, time::Duration};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer, trace::TraceLayer};

use crate::domain::issuance::CertificateIssuer;
use crate::http::handlers;

pub fn build_router(issuer: Arc<CertificateIssuer>) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/ready", get(handlers::ready))
        .route("/v1/certificates", get(handlers::list_certificates))
        .route("/v1/certificates/:id", get(handlers::get_certificate))
        .route("/v1/certificates:issue", post(handlers::issue_certificate))
        // Body limits are part of security. A CSR is small, so accepting large
        // bodies only increases memory pressure and abuse potential.
        .layer(RequestBodyLimitLayer::new(64 * 1024))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(issuer)
}
