use super::SourceCreate;
use crate::errors::DomainError;
use crate::models::source::{SourceType, SourceTypeValue};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait SourceRepository: Send + Sync {
    async fn add(&self, draft: SourceCreate) -> Result<(), DomainError>;
    async fn get_by_id(&self, id: Uuid) -> Result<SourceType, DomainError>;
    async fn get_all(&self) -> Result<Vec<SourceType>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_with_type(
        &self,
        id: Uuid,
        source_type: SourceTypeValue,
    ) -> Result<(), DomainError>;
}
