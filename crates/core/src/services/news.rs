use crate::errors::Internal;
use crate::models::news::News;
use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("Failed to update news item with title={title} and id={id}: {error}")]
    UpdateItem {
        id: Uuid,
        title: String,
        #[source]
        error: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("Failed to update news: {0}")]
    Internal(Internal),
}

#[async_trait]
pub trait NewsService: Send + Sync {
    async fn update(&self, news: &[Arc<impl News>]) -> Result<(), UpdateError>;
}
