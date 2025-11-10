pub mod atom;
pub mod telegram;

use self::atom::AtomSource;
use self::telegram::TelegramSource;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[async_trait]
pub trait Source: Send + Sync {
    fn source_id(&self) -> Uuid;
    fn source_type(&self) -> String;

    async fn watch_updates(&self);
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SourceEnum {
    Atom(AtomSource),
    Telegram(TelegramSource),
}

#[derive(Debug, Deserialize, Serialize, Type, Clone)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum SourceType {
    Atom,
    Telegram,
}
