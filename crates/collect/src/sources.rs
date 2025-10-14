pub mod atom;
pub mod telegram;

use async_trait::async_trait;

#[async_trait]
pub trait Source: Send + Sync {
    async fn watch_updates(&self);
}
