use async_trait::async_trait;
use reqwest::{Client, Response};
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

#[async_trait]
pub trait HttpService: Send + Sync {
    // @todo: The `HttpService` method `refresh_by_schedule` was replaced by `request_by_schedule` which now returns `Response` instead of `String`. Ensure the consuming code expects an HTTP response object and handles it appropriately, including reading and closing the body to avoid connection resource leaks.
    async fn request_by_schedule(&self, url: Url) -> Result<Response, String>;
}

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

        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(resp)
    }
}
