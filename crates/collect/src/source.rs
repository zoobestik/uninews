mod atom;
mod telegram;

pub use atom::*;
pub use telegram::*;

use async_trait::async_trait;
use url::Url;

#[async_trait]
pub trait Source: Send + Sync {
    fn original_url(&self) -> &Url;

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
