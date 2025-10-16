use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;

#[derive(Debug)]
pub struct News {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub url: String,
    pub image: Option<String>,
    pub published_at: Option<String>,
}

#[async_trait]
pub trait NewsService: Send + Sync {
    async fn update_news(&self, new: Vec<News>);
}

pub struct LiveNewsService {}

impl LiveNewsService {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for LiveNewsService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NewsService for LiveNewsService {
    async fn update_news(&self, content: Vec<News>) {
        info!("[content=\"{content:?}\"]");
    }
}
