//! HTTP handler modules.
//!
//! Handlers are intentionally kept thin: they own Axum extraction/response
//! concerns and delegate validation, parsing, and persistence decisions to
//! service modules. That separation prevents route code from becoming the
//! place where every business rule accumulates.
pub mod certificate;
pub mod health;
