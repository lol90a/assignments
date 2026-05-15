//! Infrastructure adapters for Assignment 2.
//!
//! Adapters implement domain ports for real external systems. The current
//! project includes PostgreSQL persistence and a dummy CA, but the same module
//! boundary can host Vault PKI, AWS Private CA, or HSM-backed implementations.
pub mod postgres;
pub mod signing;
