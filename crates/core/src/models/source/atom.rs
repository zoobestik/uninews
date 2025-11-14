use crate::models::ExternalEntity;
use chrono::{DateTime, Utc};
use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct AtomSource {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,

    pub url: Url,
}

impl AtomSource {
    #[must_use]
    pub const fn new(id: Uuid, created_at: DateTime<Utc>, url: Url) -> Self {
        Self {
            id,
            url,
            created_at,
        }
    }
}

pub struct AtomDraft {
    pub url: Url,
}

impl AtomDraft {
    #[must_use]
    pub fn new(url: Url) -> Self {
        Self { url }
    }
}

impl ExternalEntity for AtomDraft {
    fn source_key(&self) -> &str {
        self.url.as_str()
    }
}
