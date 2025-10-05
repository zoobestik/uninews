use crate::sources::{Atom, Source, TelegramChannel};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    #[serde(rename = "atom")]
    atoms: Vec<Atom>,

    #[serde(default)]
    #[serde(rename = "telegram")]
    telegrams: Vec<TelegramChannel>,
}

impl Config {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            atoms: vec![],
            telegrams: vec![],
        }
    }

    pub fn list(&'_ self) -> impl Iterator<Item = Source<'_>> {
        {
            let iter_atoms = self.atoms.iter().map(Source::Atom);
            let iter_telegrams = self.telegrams.iter().map(Source::TelegramChannel);

            iter_atoms.chain(iter_telegrams)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
