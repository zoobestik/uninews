use std::fmt::{Display, Formatter};

pub mod atom;
pub mod telegram;

pub enum SourceType {
    TelegramChannel,
    Atom,
}

impl Display for SourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TelegramChannel => write!(f, "telegram_channel_v1"),
            Self::Atom => write!(f, "atom_feed_v1"),
        }
    }
}
