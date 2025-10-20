pub mod atom;

use crate::models::atom::AtomSource;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[async_trait]
pub trait Source: Send + Sync {
    fn source_id(&self) -> Uuid;
    fn source_type(&self) -> String;

    async fn watch_updates(&self);
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SourceType {
    Atom(AtomSource),
}

impl Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Atom(_) => "atom".to_string(),
            }
        )
    }
}
