use super::Source;

use async_trait::async_trait;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub enum RefreshPeriod {
    Seconds(usize),
}

#[derive(Debug)]
pub struct Atom {
    source_url: Url,

    #[allow(dead_code)] // @ToDo: Implement refreshing
    refresh_period: RefreshPeriod,
}

impl Atom {
    fn validate_source_url(source_url: &str) -> Result<Url, String> {
        let url = Url::parse(source_url).map_err(|e| format!("{e}"))?;
        Ok(url)
    }

    /// Creates a new Atom source instance from a URL.
    ///
    /// # Arguments
    /// * `source_url` - The URL of the Atom feed
    ///
    /// # Errors
    /// Never returns an error but uses Result for consistency with other source types
    pub fn try_new(source_url: &str, refresh_period: RefreshPeriod) -> Result<Self, String> {
        let source_url = Self::validate_source_url(source_url)
            .map_err(|e| format!("[source_url=\"{source_url}\"] {e} "))?;

        Ok(Self {
            source_url,
            refresh_period,
        })
    }
}

#[async_trait]
impl Source for Atom {
    async fn watch_updates(&self) {
        println!("Running source: {}", self.source_url);
    }
}
