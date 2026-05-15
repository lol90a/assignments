use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use certificate_service::routes::router;
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn health_endpoint_returns_ok() {
    // The health route does not touch the database, so a lazy pool is enough to
    // exercise routing without requiring PostgreSQL for this test.
    let pool = PgPoolOptions::new()
        .connect_lazy("postgres://postgres:postgres@127.0.0.1:5432/certificates")
        .unwrap();
    let app = router(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
