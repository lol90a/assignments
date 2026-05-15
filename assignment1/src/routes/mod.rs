use axum::Router;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

use crate::db::DbPool;
use crate::handlers::{certificate, health};

pub fn router(pool: DbPool) -> Router {
    // All routes share one database pool through Axum state. TraceLayer adds
    // request spans without each handler needing explicit logging boilerplate.
    Router::new()
        .route("/health", get(health::health_check))
        .route(
            "/certificates",
            get(certificate::list_certificates).post(certificate::create_certificate),
        )
        .route("/certificates/:id", get(certificate::get_certificate))
        .route("/parse-certificate", post(certificate::parse_certificate))
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}
