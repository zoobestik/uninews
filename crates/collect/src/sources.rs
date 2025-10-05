mod atom;
mod telegram;

pub use self::atom::Atom;
pub use self::telegram::TelegramChannel;

pub trait SourceOrigin {
    fn original_url(&self) -> String;
}

#[derive(Debug)]
pub enum Source<'a> {
    Atom(&'a Atom),
    TelegramChannel(&'a TelegramChannel),
}
