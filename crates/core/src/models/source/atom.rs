use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomSource {
    pub id: Uuid,
    pub source: String,
    pub created_at: DateTime<Utc>,
}
