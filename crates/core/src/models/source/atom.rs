use crate::models::SourceTypeValue;
use crate::uuid::gen_consistent_uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct AtomSource {
    pub id: Uuid,
    pub source: SourceTypeValue,
    pub created_at: DateTime<Utc>,

    pub url: Url,
}

impl AtomSource {
    #[must_use]
    pub const fn new(id: Uuid, created_at: DateTime<Utc>, url: Url) -> Self {
        Self {
            id,
            url,
            source: SourceTypeValue::Atom,
            created_at,
        }
    }
}

pub struct AtomDraft {
    pub url: Url,
    pub source_id: Uuid,
}

impl AtomDraft {
    #[must_use]
    pub fn new(url: Url) -> Self {
        Self {
            source_id: gen_consistent_uuid(&Uuid::now_v7(), url.as_str()),
            url,
        }
    }
}
