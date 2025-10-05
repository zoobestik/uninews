use crate::sources::SourceOrigin;
use serde::{Deserialize, Deserializer};
use url::Url;

#[derive(Deserialize)]
struct TelegramChannelRaw {
    nickname: String,
}

impl TelegramChannelRaw {
    fn validate_nickname(name: &str) -> Result<(), String> {
        if name.len() > 32 {
            return Err(format!(
                "{name} is too long for a nickname; it must be less than 32 characters."
            ));
        }

        if name.len() < 5 {
            return Err(format!(
                "{name} is too short for a nickname; it must be at least 5 characters."
            ));
        }

        let has_invalid_chars = name
            .chars()
            .any(|c| !(c.is_ascii_alphanumeric() || c == '_'));

        if has_invalid_chars {
            return Err(format!(
                "{name} is an invalid nickname. It must contain only letters, numbers, and underscores."
            ));
        }

        Ok(())
    }

    fn validate(self) -> Result<Self, String> {
        Self::validate_nickname(&self.nickname)?;
        Ok(self)
    }
}

#[derive(Debug)]
pub struct TelegramChannel {
    // name: String,
    url: Url,
}

impl<'de> Deserialize<'de> for TelegramChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let channel = TelegramChannelRaw::deserialize(deserializer)?
            .validate()
            .map_err(serde::de::Error::custom)?;

        let url = Url::parse(&format!("https://t.me/{}", channel.nickname))
            .map_err(serde::de::Error::custom)?;

        Ok(Self {
            url,
            /*name: channel.name*/
        })
    }
}

impl SourceOrigin for TelegramChannel {
    fn original_url(&self) -> String {
        self.url.to_string()
    }
}
