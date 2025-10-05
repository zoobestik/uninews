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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_and_match_source_variants() {
        let atom: Atom = toml::from_str(r#"url = "https://a.test/rss""#).unwrap();
        let tg: TelegramChannel = toml::from_str(r#"nickname = "abcde""#).unwrap();

        match Source::Atom(&atom) {
            Source::Atom(a) => assert_eq!(a.original_url(), "https://a.test/rss"),
            Source::TelegramChannel(_) => panic!("expected Atom variant"),
        }

        match Source::TelegramChannel(&tg) {
            Source::TelegramChannel(t) => assert_eq!(t.original_url(), "https://t.me/abcde"),
            Source::Atom(_) => panic!("expected TelegramChannel variant"),
        }
    }

    fn as_origin_url(o: &dyn SourceOrigin) -> String {
        o.original_url()
    }

    #[test]
    fn source_origin_calls_work_for_variants() {
        let atom: Atom = toml::from_str(r#"url = "https://a.test/rss""#).unwrap();
        let tg: TelegramChannel = toml::from_str(r#"nickname = "rust_lang""#).unwrap();

        assert_eq!(as_origin_url(&atom), "https://a.test/rss");
        assert_eq!(as_origin_url(&tg), "https://t.me/rust_lang");
    }
}
