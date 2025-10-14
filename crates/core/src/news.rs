use async_trait::async_trait;
use tracing::info;

#[async_trait]
pub trait NewsService: Send + Sync {
    async fn update_news(&self, content: String);
}

pub struct LiveNewsService {}

impl LiveNewsService {
    #[must_use]
    pub const fn try_new() -> Self {
        Self {}
    }
}

#[async_trait]
impl NewsService for LiveNewsService {
    async fn update_news(&self, content: String) {
        info!("[content=\"{content}\"]");
    }
}
