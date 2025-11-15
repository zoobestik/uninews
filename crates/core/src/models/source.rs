pub mod atom;
pub mod telegram;

use self::atom::AtomSource;
use self::telegram::TelegramSource;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Source: Send + Sync {
    fn source_key(&self) -> Uuid;
    fn source_type(&self) -> String;

    async fn watch_updates(&self);
}

#[derive(Debug)]
pub enum SourceEnum {
    Atom(AtomSource),
    Telegram(TelegramSource),
}
