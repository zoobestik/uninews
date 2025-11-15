use crate::errors::InvalidArgument;
use crate::models::ExternalEntity;
use chrono::{DateTime, Utc};
use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct TelegramSource {
    pub id: Uuid,
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
            created_at,
            username,
            public_url,
        })
    }
}

pub struct TelegramDraft {
    pub username: String,
}

impl TelegramDraft {
    #[must_use]
    pub fn new(username: String) -> Self {
        Self { username }
    }
}

impl ExternalEntity for TelegramDraft {
    fn source_key(&self) -> &str {
        self.username.as_str()
    }
}
