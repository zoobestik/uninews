use crate::models::source::atom::AtomDraft;
use crate::models::source::telegram::TelegramDraft;
use uuid::Uuid;

pub mod news;
pub mod source;

pub enum SourceDraft {
    Atom(AtomDraft),
    Telegram(TelegramDraft),
}

impl SourceDraft {
    pub fn id(&self) -> Uuid {
        match self {
            Self::Atom(draft) => draft.source_id,
            Self::Telegram(draft) => draft.source_id,
        }
    }
}
