use super::SourceOrigin;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Atom {
    url: Url,
}

impl SourceOrigin for Atom {
    fn original_url(&self) -> String {
        self.url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_valid_absolute_url() {
        let toml_str = r#"url = "https://example.com/feed.xml""#;
        let atom: Atom = toml::from_str(toml_str).expect("should deserialize Atom with valid URL");
        assert_eq!(atom.original_url(), "https://example.com/feed.xml");
    }

    #[test]
    fn rejects_invalid_or_relative_url() {
        assert!(toml::from_str::<Atom>(r#"url = "./feed.xml""#).is_err());
        assert!(toml::from_str::<Atom>(r#"url = "not a url""#).is_err());
    }

    #[test]
    fn original_url_returns_exact_to_string() {
        assert_eq!(
            toml::from_str::<Atom>(r#"url = "https://a.b/c?x=1#frag""#)
                .unwrap()
                .original_url(),
            Url::parse("https://a.b/c?x=1#frag").unwrap().to_string()
        );
    }
}
