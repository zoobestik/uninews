use async_trait::async_trait;
use reqwest::Response;
use url::Url;

#[async_trait]
pub trait HttpService: Send + Sync {
    async fn request_by_schedule(&self, url: Url) -> Result<Response, String>;
}
