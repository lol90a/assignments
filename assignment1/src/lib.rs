//! Library entry point for Assignment 1.
//!
//! The binary and integration tests both use these modules, so the crate root
//! exposes clean boundaries instead of hiding all behavior inside `main.rs`.
//! This keeps the service testable and makes the HTTP, persistence, and domain
//! layers independently reviewable.
pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
