use async_trait::async_trait;
use reqwest::Response;
use url::Url;

#[async_trait]
pub trait HttpService: Send + Sync {
    // @todo: The `HttpService` method `refresh_by_schedule` was replaced by `request_by_schedule` which now returns `Response` instead of `String`. Ensure the consuming code expects an HTTP response object and handles it appropriately, including reading and closing the body to avoid connection resource leaks.
    async fn request_by_schedule(&self, url: Url) -> Result<Response, String>;
}
