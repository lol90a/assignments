//! Lightweight health endpoint.
//!
//! This route deliberately avoids database access so container orchestrators can
//! tell whether the process is alive without turning a temporary PostgreSQL
//! outage into an immediate liveness failure.
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub async fn health_check() -> impl IntoResponse {
    // A tiny JSON payload is friendlier to load balancers, curl, and tests than
    // an empty 200, while still avoiding expensive downstream checks.
    Json(HealthResponse { status: "ok" })
}
