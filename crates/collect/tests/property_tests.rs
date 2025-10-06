use feed_collect::sources::{Atom, TelegramChannel};
use proptest::prelude::*;
use toml;

proptest! {
    // Generate valid Telegram nicknames: [A-Za-z0-9_]{5,32}
    #[test]
    fn prop_valid_telegram_nicknames_are_accepted(nickname in "[A-Za-z0-9_]{5,32}") {
        let s = format!("nickname = \"{}\"", nickname);
        // Deserialization should succeed
        let tg: Result<TelegramChannel, _> = toml::from_str(&s);
        prop_assert!(tg.is_ok());
    }

    // Invalid nicknames: too short, too long, or contains invalid chars
    #[test]
    fn prop_invalid_telegram_nicknames_rejected(nickname in any::<String>()) {
        // Quickly filter out trivially valid ones to focus on likely-invalids
        let valid = nickname.len() >= 5 && nickname.len() <= 32 && nickname.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        prop_assume!(!valid);

        let s = format!("nickname = \"{}\"", nickname);
        let tg: Result<TelegramChannel, _> = toml::from_str(&s);
        prop_assert!(tg.is_err());
    }

    // For Atom URLs, ensure http(s) absolute URLs pass
    #[test]
    fn prop_http_urls_are_accepted(path in "[A-Za-z0-9_./-]{1,30}") {
        // Generate some simple http(s) urls with safe path characters
        let url = format!("https://example.com/{}", path);
        let s = format!("url = \"{}\"", url);
        let atom: Result<Atom, _> = toml::from_str(&s);
        prop_assert!(atom.is_ok());
    }

    // Random strings without scheme should be rejected
    #[test]
    fn prop_non_url_strings_rejected(s in "[A-Za-z0-9_./?#-]{1,40}") {
        prop_assume!(!s.contains("://"));
        let s = format!("url = \"{}\"", s);
        let atom: Result<Atom, _> = toml::from_str(&s);
        prop_assert!(atom.is_err());
    }
}
