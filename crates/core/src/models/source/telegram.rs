use super::SourceTypeValue;
use crate::uuid::gen_consistent_uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramChannelSource {
    pub id: Uuid,
    pub source: SourceTypeValue,
    pub created_at: DateTime<Utc>,

    pub username: String,
}

impl TelegramChannelSource {
    #[must_use]
    pub const fn new(id: Uuid, username: String, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            source: SourceTypeValue::Telegram,
            created_at,
            username,
        }
    }
}

pub struct TelegramChannelDraft {
    pub username: String,
    pub source_id: Uuid,
}

static TELEGRAM_UUID: Uuid = Uuid::from_u128(0x0000_0000_0000_0000_0000_0000_0000_0002);

impl TelegramChannelDraft {
    #[must_use]
    pub fn new(username: String) -> Self {
        Self {
            source_id: gen_consistent_uuid(&TELEGRAM_UUID, username.as_str()),
            username,
        }
    }
}
