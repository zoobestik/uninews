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
