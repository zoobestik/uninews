use super::News;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait NewsService: Send + Sync {
    async fn update_news(&self, news: &[Arc<dyn News>]) -> Result<(), String>;
}
