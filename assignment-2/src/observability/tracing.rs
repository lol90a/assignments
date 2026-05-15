use tracing_subscriber::{fmt, EnvFilter};

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info,sqlx=warn"));

    // JSON logs are easier for production log backends to parse and correlate.
    // Human-readable logs are pleasant locally, but production systems need
    // stable fields more than pretty formatting.
    fmt()
        .json()
        .with_env_filter(filter)
        .with_current_span(true)
        .with_span_list(true)
        .init();
}
