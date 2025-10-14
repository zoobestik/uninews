pub mod atom;
pub mod telegram;

use async_trait::async_trait;
use atom::Atom;
use telegram::TelegramChannel;

#[async_trait]
pub trait Source: Send + Sync {
    async fn watch_updates(&self);
}

pub enum SourceKind {
    Atom(Atom),
    Telegram(TelegramChannel),
}

impl SourceKind {
    #[must_use]
    pub fn as_source(&self) -> &dyn Source {
        match self {
            Self::Atom(x) => x,
            Self::Telegram(x) => x,
        }
    }
}
