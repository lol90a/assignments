use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub async fn health_check() -> impl IntoResponse {
    Json(HealthResponse { status: "ok" })
}
