use super::SourceRepository;
use crate::models::SourceType;
use crate::models::atom::AtomSource;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool, query_as};
use uuid::Uuid;

#[allow(dead_code)] // @todo: implement
struct SqliteSourceRepository {
    db_pool: SqlitePool,
}

impl SqliteSourceRepository {
    #[allow(dead_code)] // @todo: implement
    pub const fn new(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }
}

#[derive(FromRow)]
struct SourceQueryResult {
    id: Uuid,
    source: String,
    created_at: DateTime<Utc>,
}

impl TryFrom<SourceQueryResult> for SourceType {
    type Error = String;

    fn try_from(source: SourceQueryResult) -> Result<Self, Self::Error> {
        Ok(Self::Atom(AtomSource {
            id: source.id,
            source: source.source.to_string(),
            created_at: source.created_at,
        }))
    }
}

impl SourceRepository for SqliteSourceRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<SourceType, String> {
        let source = query_as!(
            SourceQueryResult,
            r#"
            SELECT
                id as "id: Uuid",
                source,
                created_at as "created_at: DateTime<Utc>"
            FROM sources
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Source not found".to_string())?;

        source.try_into()
    }

    async fn find_by_ext_id(&self, id: Uuid) -> Result<SourceType, String> {
        let source = query_as!(
            SourceQueryResult,
            r#"
            SELECT
                id as "id: Uuid",
                source,
                created_at as "created_at: DateTime<Utc>"
            FROM sources
            WHERE source_id = ?
            "#,
            id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Source not found".to_string())?;

        source.try_into()
    }
}
