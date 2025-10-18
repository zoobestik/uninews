use super::service::StorageService;
use crate::fs::write_to_file;
use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

pub struct LiveStorageService {}

impl LiveStorageService {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    async fn save(&self, key: &str, value: &str) -> Result<(), String> {
        let path_string = format!("out/cache/{key}.html");
        write_to_file(path_string.as_str(), value).await
    }
}

impl Default for LiveStorageService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StorageService for LiveStorageService {
    async fn save_raw(&self, key: Uuid, value: &str) {
        if let Err(e) = self.save(&key.to_string(), value).await {
            error!("Failed to save raw: {e}");
        }
    }
}
