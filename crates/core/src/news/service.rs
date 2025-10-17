use super::News;
use async_trait::async_trait;

#[async_trait]
pub trait NewsService: Send + Sync {
    async fn update_news(&self, new: Vec<&dyn News>);
}
