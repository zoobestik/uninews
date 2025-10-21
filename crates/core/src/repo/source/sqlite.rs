use super::{AtomDraft, SourceCreate, SourceRepository, TelegramChannelDraft};
use crate::models::atom::AtomSource;
use crate::models::telegram::TelegramChannelSource;
use crate::models::{SourceType, SourceTypeValue};
use async_trait::async_trait;
use sqlx::error::BoxDynError;
use sqlx::sqlite::SqliteValueRef;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{Decode, FromRow, Sqlite, SqlitePool, query, query_as};
use std::ops::Deref;
use url::Url as UrlLib;
use uuid::Uuid;

pub struct SqliteSourceRepository {
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
        let inner_url = url::Url::parse(&url_str)?;
        Ok(Self(inner_url))
    }
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

    url: Option<Url>,
}

impl TryFrom<SourceQueryResult> for SourceType {
    type Error = String;

    fn try_from(source: SourceQueryResult) -> Result<Self, Self::Error> {
        Ok(match source.source {
            SourceTypeValue::Atom => Self::Atom(AtomSource::new(
                source.id,
                source.created_at,
                source
                    .url
                    .ok_or_else(|| "Missing URL for Atom source".to_string())?
                    .0,
            )),
            SourceTypeValue::TelegramChannel => {
                Self::TelegramChannel(TelegramChannelSource::new(source.id, source.created_at))
            }
        })
    }
}

impl SqliteSourceRepository {
    async fn insert_atom(&self, draft: AtomDraft) -> Result<(), String> {
        let id = Uuid::now_v7();

        let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;

        let atom_source = query!(
            r#"
            INSERT INTO sources (id, source)
            VALUES ($1, $2)
            RETURNING
                id as "id: Uuid"
            "#,
            id,
            SourceTypeValue::Atom
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let url = draft.url.as_str();

        query!(
            r#"
            INSERT INTO source_atom_details (atom_details_id, url)
            VALUES ($1, $2)
            "#,
            atom_source.id,
            url,
        )
        .execute(&mut *tx)
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

        Ok(())
    }

    async fn insert_telegram_channel(&self, draft: TelegramChannelDraft) -> Result<(), String> {
        let id = Uuid::now_v7();

        query_as!(
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

        Ok(())
    }
}

#[async_trait]
impl SourceRepository for SqliteSourceRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<SourceType, String> {
        query_as!(
            SourceQueryResult,
            r#"
            SELECT
                s.id as "id: Uuid",
                s.source as "source: SourceTypeValue",
                s.created_at as "created_at: DateTime<Utc>",
                d.url as "url: Url"
            FROM sources s
            LEFT JOIN
                source_atom_details d ON s.id = d.atom_details_id
            WHERE
                s.id = ?
            "#,
            id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Source not found".to_string())?
        .try_into()
    }

    async fn find_all_sources(&self) -> Result<Vec<SourceType>, String> {
        let news = query_as!(
            SourceQueryResult,
            r#"
            SELECT
                s.id as "id: Uuid",
                s.source as "source: SourceTypeValue",
                s.created_at as "created_at: DateTime<Utc>",
                d.url as "url: Url"
            FROM sources s
            LEFT JOIN
                source_atom_details d ON s.id = d.atom_details_id
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

    async fn insert_or_update(&self, draft: SourceCreate) -> Result<(), String> {
        match draft {
            SourceCreate::Atom(draft) => self.insert_atom(draft).await?,
            SourceCreate::TelegramChannel(draft) => self.insert_telegram_channel(draft).await?,
        }

        Ok(())
    }
}
