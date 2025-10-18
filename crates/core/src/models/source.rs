use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Source: Send + Sync {
    fn source_id(&self) -> Uuid;
    fn source_type(&self) -> String;

    async fn watch_updates(&self);
}
