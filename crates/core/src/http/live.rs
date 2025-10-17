use super::service::HttpService;
use async_trait::async_trait;
use reqwest::{Client, Response};
use std::time::Duration;
use tokio::time::sleep;
use tracing::debug;
use url::Url;

pub struct LiveHttpService {
    client: Client,
}

impl LiveHttpService {
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Default for LiveHttpService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HttpService for LiveHttpService {
    async fn request_by_schedule(&self, url: Url) -> Result<Response, String> {
        sleep(Duration::from_secs(10)).await;

        let url_str = url.to_string();
        debug!("[http_service=\"{0}\"] fetching url", url_str);

        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        debug!("[http_service=\"{0}\"] fetched url", url_str);

        Ok(resp)
    }
}
