use super::{AtomDraft, CreateSource, SourceRepository, TelegramChannelDraft};
use crate::models::atom::AtomSource;
use crate::models::telegram::TelegramChannelSource;
use crate::models::{SourceType, SourceTypeValue};
use async_trait::async_trait;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool, query, query_as};
use uuid::Uuid;

pub struct SqliteSourceRepository {
    db_pool: SqlitePool,
}

impl SqliteSourceRepository {
    #[must_use]
    pub const fn new(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }
}

#[derive(FromRow)]
struct SourceQueryResult {
    id: Uuid,
    source: SourceTypeValue,
    created_at: DateTime<Utc>,
}

impl TryFrom<SourceQueryResult> for SourceType {
    type Error = String;

    fn try_from(source: SourceQueryResult) -> Result<Self, Self::Error> {
        Ok(match source.source {
            SourceTypeValue::Atom => Self::Atom(AtomSource::new(source.id, source.created_at)),
            SourceTypeValue::TelegramChannel => {
                Self::TelegramChannel(TelegramChannelSource::new(source.id, source.created_at))
            }
        })
    }
}

impl SqliteSourceRepository {
    async fn insert_atom(&self, draft: AtomDraft) -> Result<SourceType, String> {
        let id = Uuid::now_v7();

        let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;

        let atom_source = query_as!(
            AtomSource,
            r#"
            INSERT INTO sources (id, source)
            VALUES ($1, $2)
            RETURNING
                id as "id: Uuid",
                source as "source: SourceTypeValue",
                created_at as "created_at: DateTime<Utc>"
            "#,
            id,
            SourceTypeValue::Atom
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        query!(
            r#"
            INSERT INTO uuid_mappings (internal_id, external_id)
            VALUES ($1, $2)
            "#,
            atom_source.id,
            draft.source_id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(SourceType::Atom(atom_source))
    }

    async fn insert_telegram_channel(
        &self,
        draft: TelegramChannelDraft,
    ) -> Result<SourceType, String> {
        let id = Uuid::now_v7();

        let atom_source = query_as!(
            TelegramChannelSource,
            r#"
            INSERT INTO sources (id, source)
            VALUES ($1, $2)
            RETURNING
                id as "id: Uuid",
                source as "source: SourceTypeValue",
                created_at as "created_at: DateTime<Utc>"
            "#,
            id,
            SourceTypeValue::TelegramChannel,
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        query!(
            r#"
            INSERT INTO uuid_mappings (internal_id, external_id)
            VALUES ($1, $2)
            "#,
            id,
            draft.source_id,
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(SourceType::TelegramChannel(atom_source))
    }
}

#[async_trait]
impl SourceRepository for SqliteSourceRepository {
    async fn insert_or_update(&self, draft: CreateSource) -> Result<SourceType, String> {
        Ok(match draft {
            CreateSource::Atom(draft) => self.insert_atom(draft).await?,
            CreateSource::TelegramChannel(draft) => self.insert_telegram_channel(draft).await?,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<SourceType, String> {
        query_as!(
            SourceQueryResult,
            r#"
            SELECT
                id as "id: Uuid",
                source as "source: SourceTypeValue",
                created_at as "created_at: DateTime<Utc>"
            FROM sources
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Source not found".to_string())?
        .try_into()
    }
}
