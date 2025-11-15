use async_trait::async_trait;
use news_core::models::news::News;
use uuid::Uuid;

#[derive(Debug)]
pub struct TelegramItem {
    pub parent_id: Uuid,
    pub source_key: String,

    pub title: String,
    pub description: String,
}

#[async_trait]
impl News for TelegramItem {
    fn source_key(&self) -> &str {
        self.source_key.as_str()
    }
    fn parent_id(&self) -> Uuid {
        self.parent_id
    }
    fn title(&self) -> &str {
        self.title.as_str()
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn content(&self) -> &Option<String> {
        &None
    }
}
