use super::SourceDraft;
use crate::errors::Internal;
use crate::models::source::{SourceEnum, SourceType};
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct AddError(#[from] pub Internal);

#[derive(Error, Debug)]
pub enum GetError {
    #[error("Not found")]
    NotFound { id: String, entity: String },

    #[error(transparent)]
    Internal(#[from] Internal),
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct GetAllError(#[from] pub Internal);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct DropError(#[from] pub Internal);

#[async_trait]
pub trait SourceRepository: Send + Sync {
    async fn add(&self, draft: SourceDraft) -> Result<(), AddError>;
    async fn get_by_id(&self, id: Uuid) -> Result<SourceEnum, GetError>;
    async fn get_all(&self) -> Result<Vec<SourceEnum>, GetAllError>;
    async fn drop_by_id(&self, id: Uuid) -> Result<(), DropError>;
    async fn drop_by_id_and_type(&self, id: Uuid, source_type: SourceType)
    -> Result<(), DropError>;
}
