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
            atoms: Vec::new(),
            telegrams: Vec::new(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sources::{Source, SourceOrigin};

    #[test]
    fn config_list_default_is_empty() {
        assert_eq!(Config::default().list().count(), 0);
        assert_eq!(Config::new().list().count(), 0);
        assert!(
            Config::new().list().next().is_none(),
            "new() should produce empty list"
        );
    }

    // language=TOML
    const MINIMAL_VALID_TOML_STR: &str = r#"
[[atom]]
url = "https://example.com/feed.xml"

[[telegram]]
nickname = "rustaceans"
"#;

    #[test]
    fn deserialize_minimal_toml() {
        let cfg: Config = toml::from_str(MINIMAL_VALID_TOML_STR).expect("valid minimal config");

        let items: Vec<_> = cfg.list().collect();

        assert_eq!(items.len(), 2);

        match &items[0] {
            Source::Atom(a) => assert_eq!(a.original_url(), "https://example.com/feed.xml"),
            Source::TelegramChannel(_) => panic!("first should be Atom"),
        }

        match &items[1] {
            Source::TelegramChannel(t) => assert_eq!(t.original_url(), "https://t.me/rustaceans"),
            Source::Atom(_) => panic!("second should be TelegramChannel"),
        }
    }

    // language=TOML
    const WITH_MANY_ENTRIES_TOML_STR: &str = r#"
[[atom]]
url = "https://a.com/1"
[[atom]]
url = "https://a.com/2"

[[telegram]]
nickname = "aaaaa"
[[telegram]]
nickname = "bbbbb"
"#;

    const WITH_MANY_ENTRIES_URLS: &[&str] = &[
        "https://a.com/1",
        "https://a.com/2",
        "https://t.me/aaaaa",
        "https://t.me/bbbbb",
    ];

    #[test]
    fn deserialize_with_many_entries() {
        let cfg: Config = toml::from_str(WITH_MANY_ENTRIES_TOML_STR).expect("valid config");

        let urls: Vec<String> = cfg
            .list()
            .map(|s| match s {
                Source::Atom(a) => a.original_url(),
                Source::TelegramChannel(t) => t.original_url(),
            })
            .collect();

        assert_eq!(
            urls,
            WITH_MANY_ENTRIES_URLS
                .iter()
                .map(|&s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    // language=TOML
    const DESERIALIZE_FAILURES_BAD_TG: &str = r"[[telegram]]
nickname = false
";

    // language=TOML
    const DESERIALIZE_FAILURES_BAD_ATOM: &str = r"[[atom]]
url = 1
";

    #[test]
    fn deserialize_failures() {
        assert!(toml::from_str::<Config>(DESERIALIZE_FAILURES_BAD_ATOM).is_err());
        assert!(toml::from_str::<Config>(DESERIALIZE_FAILURES_BAD_TG).is_err());
        assert!(toml::from_str::<Config>("[telegram\n nickname = 'abc'").is_err());
    }
}
