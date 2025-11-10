use super::SourceType;
use super::SourceType::Telegram;
use crate::errors::InvalidArgument;
use crate::uuid::gen_consistent_uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramSource {
    pub id: Uuid,
    pub source: SourceType,
    pub created_at: DateTime<Utc>,

    pub username: String,
    pub public_url: Url,
}

impl TelegramSource {
    pub fn new(
        id: Uuid,
        username: String,
        created_at: DateTime<Utc>,
    ) -> Result<Self, InvalidArgument> {
        let public_url =
            Url::parse(&format!("https://t.me/s/{username}")).map_err(|e| InvalidArgument {
                name: "username".to_string(),
                value: username.to_string(),
                reason: format!("Invalid telegram channel name: {username}. {e}"),
            })?;

        Ok(Self {
            id,
            source: Telegram,
            created_at,
            username,
            public_url,
        })
    }
}

pub struct TelegramDraft {
    pub username: String,
    pub source_id: Uuid,
}

static TELEGRAM_UUID: Uuid = Uuid::from_u128(0x0000_0000_0000_0000_0000_0000_0000_0002);

impl TelegramDraft {
    #[must_use]
    pub fn new(username: String) -> Self {
        Self {
            source_id: gen_consistent_uuid(&TELEGRAM_UUID, username.as_str()),
            username,
        }
    }
}
