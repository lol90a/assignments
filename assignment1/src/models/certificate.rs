use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Stored certificate inventory row returned by repository queries and APIs.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Certificate {
    pub certificate_id: Uuid,
    pub subject: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub san_entries: Vec<String>,
}
