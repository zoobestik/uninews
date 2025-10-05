use crate::sources::SourceOrigin;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TelegramChannel {
    #[serde(rename = "nickname")]
    name: String,
}

impl TelegramChannel {
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self { name }
    }
}

impl SourceOrigin for TelegramChannel {
    fn original_url(&self) -> String {
        format!("https://t.me/{}", &self.name)
    }
}
