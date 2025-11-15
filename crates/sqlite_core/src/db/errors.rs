use crate::db::init::DBInitError;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum SqlxServiceError {
    #[error("Failed to initialize database: {0}")]
    DBInit(DBInitError),

    #[error("Failed to start transaction: {0}")]
    Transaction(#[from] sqlx::Error),

    #[error("Failed to execute sqlx in {}: {error}", self.format_location())]
    Execute {
        id: Option<Uuid>,
        identifier: Option<String>,
        #[source]
        error: sqlx::Error,
    },
}

impl SqlxServiceError {
    fn format_location(&self) -> String {
        if let Self::Execute { id, identifier, .. } = self {
            format!(
                "{}#{}",
                identifier.as_deref().unwrap_or("unknown"),
                id.map(|u| u.to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            )
        } else {
            "unknown".to_string()
        }
    }
}
