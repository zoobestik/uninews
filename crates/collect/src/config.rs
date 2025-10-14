use crate::source::{Atom, Source, TelegramChannel};

#[derive(Debug)]
pub struct Config {
    atoms_channels: Vec<Atom>,
    telegram_channels: Vec<TelegramChannel>,
}

impl Config {
    #[must_use]
    pub const fn new(atoms_channels: Vec<Atom>, telegram_channels: Vec<TelegramChannel>) -> Self {
        Self {
            atoms_channels,
            telegram_channels,
        }
    }

    pub fn list(&self) -> impl Iterator<Item = &dyn Source> {
        let atom_iter = self
            .atoms_channels
            .iter()
            .map(|atom_source| atom_source as &dyn Source);

        let rss_iter = self
            .telegram_channels
            .iter()
            .map(|rss_source| rss_source as &dyn Source);

        atom_iter.chain(rss_iter)
    }
}
