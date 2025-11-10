use crate::errors::Internal;
use async_trait::async_trait;
use reqwest::Response;
use std::sync::Arc;
use thiserror::Error;
use url::Url;

pub type HttpResponse = Response;

#[derive(Error, Debug)]
pub enum HandleError {}

#[async_trait]
pub trait HttpUpdateHandle: Send + Sync {
    fn url(&self) -> &Url;
    async fn handle(&self, response: Response) -> Result<(), HandleError>;
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct WatchError(#[from] Internal);

#[async_trait]
pub trait HttpService: Send + Sync {
    async fn watch_changes(
        &self,
        update_handler: Arc<dyn HttpUpdateHandle>,
    ) -> Result<(), WatchError>;
}
