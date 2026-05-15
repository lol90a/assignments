//! HTTP transport layer for Assignment 2.
//!
//! This module translates REST requests into domain commands. Keeping transport
//! code separate from policy code makes authorization and validation easier to
//! audit during a system design review.
pub mod handlers;
pub mod router;
