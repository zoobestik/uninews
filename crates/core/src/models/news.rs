use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait News: Send + Sync {
    fn source_id(&self) -> &str;
    fn parent_id(&self) -> Uuid;

    fn title(&self) -> &str;
    fn description(&self) -> &str;
    fn content(&self) -> &Option<String>;
}
