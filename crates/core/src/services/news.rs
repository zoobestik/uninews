mod live;
pub use live::*;

mod service;
pub use service::*;

use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait News: Send + Sync {
    fn source_id(&self) -> Uuid;
    fn parent_id(&self) -> Uuid;

    fn title(&self) -> &str;
    fn description(&self) -> &str;
    fn content(&self) -> &Option<String>;
}
