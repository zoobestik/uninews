use crate::utils::codecs::Url;
use async_trait::async_trait;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool, query, query_as};
use uninews_core::errors::DomainError;
use uninews_core::models::source::atom::{AtomDraft, AtomSource};
use uninews_core::models::source::telegram::{TelegramChannelDraft, TelegramChannelSource};
use uninews_core::models::source::{SourceType, SourceTypeValue};
use uninews_core::repos::SourceCreate;
use uninews_core::repos::source::SourceRepository;
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

    atom_url: Option<Url>,

    telegram_username: Option<String>,
}

impl TryFrom<SourceQueryResult> for SourceType {
    type Error = DomainError;

    fn try_from(source: SourceQueryResult) -> Result<Self, Self::Error> {
        Ok(match source.source {
            SourceTypeValue::Atom => Self::Atom(AtomSource::new(
                source.id,
                source.created_at,
                source
                    .atom_url
                    .ok_or_else(|| {
                        DomainError::InvalidInput("Missing URL for Atom source".to_string())
                    })?
                    .0,
            )),
            SourceTypeValue::Telegram => Self::TelegramChannel(TelegramChannelSource::new(
                source.id,
                source.telegram_username.ok_or_else(|| {
                    DomainError::InvalidInput("Missing username for Telegram source".to_string())
                })?,
                source.created_at,
            )),
        })
    }
}

impl SqliteSourceRepository {
    async fn insert_atom(&self, draft: AtomDraft) -> Result<(), DomainError> {
        let id = Uuid::now_v7();

        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let result = query!(
            r#"
            INSERT INTO uuid_mappings (internal_id, external_id)
            VALUES ($1, $2) ON CONFLICT(external_id) DO NOTHING
            "#,
            id,
            draft.source_id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            tx.rollback()
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            return Err(DomainError::Conflict(format!(
                "[atom_feed={0}] mapping {1} already exists",
                draft.url, draft.source_id
            )));
        }

        query!(
            r#"
            INSERT INTO sources (id, source)
            VALUES ($1, $2)
            "#,
            id,
            SourceTypeValue::Atom
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        let url = draft.url.as_str();

        let result = query!(
            r#"
            INSERT INTO source_atom_details (atom_details_id, url)
            VALUES ($1, $2) ON CONFLICT(url) DO NOTHING
            "#,
            id,
            url,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            tx.rollback()
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            return Err(DomainError::Conflict(format!(
                "[atom_feed={url}] mapping already exists"
            )));
        }

        tx.commit()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn insert_telegram_channel(
        &self,
        draft: TelegramChannelDraft,
    ) -> Result<(), DomainError> {
        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let id = Uuid::now_v7();

        let result = query!(
            r#"
            INSERT INTO uuid_mappings (internal_id, external_id)
            VALUES ($1, $2) ON CONFLICT(external_id) DO NOTHING
            "#,
            id,
            draft.source_id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            tx.rollback()
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            return Err(DomainError::Conflict(format!(
                "[telegram_channel={0}] mapping {1} already exists",
                draft.username, draft.source_id
            )));
        }

        query!(
            r#"
            INSERT INTO sources (id, source)
            VALUES ($1, $2)
            "#,
            id,
            SourceTypeValue::Telegram,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        let result = query!(
            r#"
            INSERT INTO source_telegram_details (telegram_details_id, username)
            VALUES ($1, $2)
            ON CONFLICT(username) DO NOTHING
            "#,
            id,
            draft.username,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            tx.rollback()
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            return Err(DomainError::Conflict(format!(
                "[telegram_channel={0}] username already exists",
                draft.username
            )));
        }

        tx.commit()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl SourceRepository for SqliteSourceRepository {
    async fn add(&self, draft: SourceCreate) -> Result<(), DomainError> {
        match draft {
            SourceCreate::Atom(draft) => self.insert_atom(draft).await?,
            SourceCreate::TelegramChannel(draft) => self.insert_telegram_channel(draft).await?,
        }

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<SourceType, DomainError> {
        query_as!(
            SourceQueryResult,
            r#"
            SELECT
                src.id as "id: Uuid",
                src.source as "source: SourceTypeValue",
                src.created_at as "created_at: DateTime<Utc>",
                -- atom details
                atom.url as "atom_url: Url",
                -- telegram details
                tg.username as "telegram_username: String"
            FROM sources src
            LEFT JOIN
                source_atom_details atom ON src.id = atom.atom_details_id
            LEFT JOIN
                source_telegram_details tg ON src.id = tg.telegram_details_id
            WHERE
                src.id = ?
            "#,
            id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .ok_or_else(|| DomainError::NotFound("Source not found".to_string()))?
        .try_into()
    }

    async fn get_all(&self) -> Result<Vec<SourceType>, DomainError> {
        let news = query_as!(
            SourceQueryResult,
            r#"
            SELECT
                src.id as "id: Uuid",
                src.source as "source: SourceTypeValue",
                src.created_at as "created_at: DateTime<Utc>",
                -- atom details
                atom.url as "atom_url: Url",
                -- telegram details
                tg.username as "telegram_username: String"
            FROM sources src
            LEFT JOIN
                source_atom_details atom ON src.id = atom.atom_details_id
            LEFT JOIN
                source_telegram_details tg ON src.id = tg.telegram_details_id
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        let sources = news
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<SourceType>, DomainError>>()?;

        Ok(sources)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<(), DomainError> {
        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        query!(
            r#"
            DELETE FROM uuid_mappings
            WHERE internal_id IN (
                SELECT id
                FROM sources
                WHERE id = $1
            )
            "#,
            id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn delete_with_type(
        &self,
        source_id: Uuid,
        source_type: SourceTypeValue,
    ) -> Result<(), DomainError> {
        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let result = query!(
            r#"
            DELETE FROM uuid_mappings
            WHERE external_id = $1 AND internal_id IN (
                SELECT id
                FROM sources
                WHERE source = $2
            )
            "#,
            source_id,
            source_type,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            tx.rollback()
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
            return Err(DomainError::NotFound(format!(
                "source {source_id} not found"
            )));
        }

        tx.commit()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}
