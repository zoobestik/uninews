use super::service::StorageService;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;
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
        let path = Path::new(path_string.as_str());

        fs::create_dir_all(
            path.parent()
                .ok_or_else(|| format!("Failed to get parent {0}", path.display()))?,
        )
        .await
        .map_err(|e| format!("Failed to create directory {0}: {e}", path.display()))?;

        fs::write(path, value)
            .await
            .map_err(|e| format!("Failed to write file [{0}]: {e}", path.display()))?;

        Ok(())
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
