//! Domain layer for the secure certificate platform.
//!
//! The domain layer contains workflow and policy concepts without importing
//! Axum, SQLx, Kubernetes, or Vault-specific code. That dependency direction is
//! what lets the same issuance rules survive infrastructure changes.
pub mod issuance;
pub mod model;
pub mod ports;
