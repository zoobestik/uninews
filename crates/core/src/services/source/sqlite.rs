use super::{AtomDraft, SourceCreate, SourceService, TelegramChannelDraft};
use crate::models::atom::AtomSource;
use crate::models::telegram::TelegramChannelSource;
use crate::models::{SourceType, SourceTypeValue};
use crate::parse::parse_url;
use async_trait::async_trait;
use sqlx::error::BoxDynError;
use sqlx::sqlite::SqliteValueRef;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{Decode, FromRow, Sqlite, SqlitePool, query, query_as};
use std::ops::Deref;
use url::Url as UrlLib;
use uuid::Uuid;

pub struct SqliteSourceService {
    db_pool: SqlitePool,
}

#[derive(Debug)]
pub struct Url(UrlLib);

impl Deref for Url {
    type Target = url::Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'r> Decode<'r, Sqlite> for Url {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let url_str = <String as Decode<Sqlite>>::decode(value)?;
        let inner_url = parse_url(&url_str)?;
        Ok(Self(inner_url))
    }
}

impl SqliteSourceService {
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
    type Error = String;

    fn try_from(source: SourceQueryResult) -> Result<Self, Self::Error> {
        Ok(match source.source {
            SourceTypeValue::Atom => Self::Atom(AtomSource::new(
                source.id,
                source.created_at,
                source
                    .atom_url
                    .ok_or_else(|| "Missing URL for Atom source".to_string())?
                    .0,
            )),
            SourceTypeValue::Telegram => Self::TelegramChannel(TelegramChannelSource::new(
                source.id,
                source
                    .telegram_username
                    .ok_or_else(|| "Missing username for Telegram source".to_string())?,
                source.created_at,
            )),
        })
    }
}

impl SqliteSourceService {
    async fn insert_atom(&self, draft: AtomDraft) -> Result<(), String> {
        let id = Uuid::now_v7();

        let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;

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
        .map_err(|e| e.to_string())?;

        let result = query!(
            r#"
            INSERT INTO uuid_mappings (internal_id, external_id)
            VALUES ($1, $2)
            ON CONFLICT(external_id) DO NOTHING
            "#,
            id,
            draft.source_id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let url = draft.url.as_str();

        if result.rows_affected() == 0 {
            return Err(format!("[atom_feed={url}] mapping already exists"));
        }

        query!(
            r#"
            INSERT INTO source_atom_details (atom_details_id, url)
            VALUES ($1, $2)
            "#,
            id,
            url,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn insert_telegram_channel(&self, draft: TelegramChannelDraft) -> Result<(), String> {
        let id = Uuid::now_v7();

        let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;

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
        .map_err(|e| e.to_string())?;

        let result = query!(
            r#"
            INSERT INTO uuid_mappings (internal_id, external_id)
            VALUES ($1, $2)
            ON CONFLICT(external_id) DO NOTHING
            "#,
            id,
            draft.source_id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            return Err(format!(
                "[telegram_channel={0}] mapping already exists",
                draft.username
            ));
        }

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
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            return Err(format!(
                "[telegram_channel={0}] username already exists",
                draft.username
            ));
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[async_trait]
impl SourceService for SqliteSourceService {
    async fn add(&self, draft: SourceCreate) -> Result<(), String> {
        match draft {
            SourceCreate::Atom(draft) => self.insert_atom(draft).await?,
            SourceCreate::TelegramChannel(draft) => self.insert_telegram_channel(draft).await?,
        }

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<SourceType, String> {
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
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Source not found".to_string())?
        .try_into()
    }

    async fn get_all(&self) -> Result<Vec<SourceType>, String> {
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
        .map_err(|e| e.to_string())?;

        let sources = news
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<SourceType>, String>>()?;

        Ok(sources)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<(), String> {
        let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;

        query!(
            r#"
            DELETE FROM sources
            WHERE id = $1
            "#,
            id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete_with_type(&self, id: Uuid, source_type: SourceTypeValue) -> Result<(), String> {
        let result = query!(
            r#"
            DELETE FROM sources
            WHERE id IN (
                SELECT internal_id
                FROM uuid_mappings
                WHERE external_id = $1
            ) AND source = $2
            "#,
            id,
            source_type,
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            return Err(format!("source {id} not found"));
        }
        Ok(())
    }
}
