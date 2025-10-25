use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uninews_core::services::news::News;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramItem {
    pub parent_id: Uuid,
    pub source_id: Uuid,
    pub description: String,
}

#[async_trait]
impl News for TelegramItem {
    fn source_id(&self) -> Uuid {
        self.source_id
    }

    fn parent_id(&self) -> Uuid {
        self.parent_id
    }

    fn title(&self) -> &'static str {
        "self.source_id.to_string().as_str()"
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn content(&self) -> &Option<String> {
        &None
    }
}
