//! Observability wiring for logs, metrics, and traces.
//!
//! Production services need telemetry initialized once at process startup. This
//! module keeps that cross-cutting concern out of request handlers so business
//! logic stays focused on certificate workflows.
pub mod tracing;
