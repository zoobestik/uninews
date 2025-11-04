use crate::models::source::atom::AtomDraft;
use crate::models::source::telegram::TelegramChannelDraft;

pub mod news;
pub mod source;

pub enum SourceCreate {
    Atom(AtomDraft),
    TelegramChannel(TelegramChannelDraft),
}
