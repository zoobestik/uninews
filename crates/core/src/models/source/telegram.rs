use crate::models::SourceTypeValue;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramChannelSource {
    pub id: Uuid,
    pub source: SourceTypeValue,
    pub created_at: DateTime<Utc>,
}

impl TelegramChannelSource {
    #[must_use]
    pub const fn new(id: Uuid, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            source: SourceTypeValue::TelegramChannel,
            created_at,
        }
    }
}

pub struct TelegramChannelDraft {
    pub source_id: Uuid,
}
