use super::Source;

use async_trait::async_trait;
use url::Url;

#[derive(Debug)]
pub struct Atom {
    source_url: Url,
}

impl Atom {
    /// Creates a new Atom source instance from a URL.
    ///
    /// # Arguments
    /// * `source_url` - The URL of the Atom feed
    ///
    /// # Errors
    /// Never returns an error but uses Result for consistency with other source types
    pub const fn try_new(source_url: Url) -> Result<Self, String> {
        Ok(Self { source_url })
    }
}

#[async_trait]
impl Source for Atom {
    fn original_url(&self) -> &Url {
        &self.source_url
    }

    async fn watch_updates(&self) {
        println!("Running source: {}", self.original_url());
    }
}
