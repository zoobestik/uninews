use super::Source;

use async_trait::async_trait;
use tracing::debug;
use url::Url;

#[derive(Debug)]
pub struct TelegramChannel {
    channel_url: Url,
}

impl TelegramChannel {
    /// Validate a Telegram channel nickname.
    ///
    /// # Arguments
    /// * `name` - The nickname to validate
    ///
    /// # Errors
    /// Returns error if:
    /// * Nickname is longer
    /// * Nickname is shorter
    /// * Nickname contains characters other than letters, numbers, and underscores
    pub fn validate_nickname(name: &str) -> Result<(), String> {
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

    /// Creates a new Telegram channel instance from a nickname.
    ///
    /// # Arguments
    /// * `name` - The nickname of the Telegram channel (5-32 characters, alphanumeric and underscores only)
    ///
    /// # Errors
    /// Returns error if:
    /// * Nickname validation fails - too short/long or contains invalid characters
    /// * Cannot construct valid URL from the nickname
    pub fn try_new(name: &str) -> Result<Self, String> {
        Self::validate_nickname(name)?;

        let channel_url = Url::parse(&format!("https://t.me/{name}")).map_err(|e| {
            debug!(error = %e);
            "Invalid channel name."
        })?;

        Ok(Self { channel_url })
    }
}

#[async_trait]
impl Source for TelegramChannel {
    async fn watch_updates(&self) {
        println!("Running sources: {}", self.channel_url);
    }
}
