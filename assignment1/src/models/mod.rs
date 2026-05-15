//! Shared data models for Assignment 1.
//!
//! Domain rows and transport DTOs are separated so database-facing structs can
//! evolve independently from API request/response shapes.
pub mod certificate;
pub mod dto;

pub use certificate::Certificate;
pub use dto::{
    CreateCertificateRequest, CreateCertificateResponse, ParseCertificateRequest,
    ParsedCertificateResponse,
};
