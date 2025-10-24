use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait StorageService: Send + Sync {
    async fn save_raw(&self, key: Uuid, value: &str);
}
