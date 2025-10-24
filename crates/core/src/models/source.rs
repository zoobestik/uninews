pub mod atom;
pub mod telegram;

use self::atom::AtomSource;
use self::telegram::TelegramChannelSource;
use async_trait::async_trait;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[async_trait]
pub trait Source: Send + Sync {
    fn source_id(&self) -> Uuid;
    fn source_type(&self) -> String;

    async fn watch_updates(&self);
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SourceType {
    Atom(AtomSource),
    TelegramChannel(TelegramChannelSource),
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, Clone, ValueEnum)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum SourceTypeValue {
    Atom,
    Telegram,
}
