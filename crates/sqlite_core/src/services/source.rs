use super::uuid::UuidGroup::{SourceAtom, SourceTelegram};
use super::uuid::{SqliteUuidService, UpsertMapping};
use crate::db::codecs::Url;
use crate::db::errors::SqlxServiceError;
use crate::db::errors::SqlxServiceError::{Execute, Transaction};
use crate::db::init::{DBInitError, init_db_pool};
use async_trait::async_trait;
use news_core::errors::InvalidArgument;
use news_core::models::ExternalEntity;
use news_core::models::source::SourceEnum;
use news_core::models::source::atom::{AtomDraft, AtomSource};
use news_core::models::source::telegram::{TelegramDraft, TelegramSource};
use news_core::services::source::{
    AddError, DropError, GetAllError, GetError, SourceDraft, SourceService,
};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool, Type, query, query_as};
use tokio::try_join;
use uuid::Uuid;

#[derive(Debug, Type)]
#[sqlx(rename_all = "snake_case")]
enum SourceType {
    Atom,
    Telegram,
}

#[derive(FromRow)]
struct SourceQueryResult {
    id: Uuid,
    source: SourceType,
    created_at: DateTime<Utc>,

    atom_url: Option<Url>,

    telegram_username: Option<String>,
}

pub struct SqliteSourceService {
    db_pool: SqlitePool,
    uuid_repo: SqliteUuidService,
}

impl SqliteSourceService {
    pub async fn try_new() -> Result<Self, DBInitError> {
        let (db_pool, uuid_repo) = try_join!(init_db_pool(), SqliteUuidService::init_lazy())?;
        Ok(Self { db_pool, uuid_repo })
    }

    async fn get_data_by_draft(
        &self,
        draft: SourceDraft,
    ) -> Result<(Uuid, SourceType), SqlxServiceError> {
        let (source_key, uuid_group, source_type) = match &draft {
            SourceDraft::Atom(draft) => (
                ExternalEntity::source_key(draft),
                SourceAtom,
                SourceType::Atom,
            ),
            SourceDraft::Telegram(draft) => (
                ExternalEntity::source_key(draft),
                SourceTelegram,
                SourceType::Telegram,
            ),
        };

        Ok((
            self.uuid_repo
                .get_by_source_key(&uuid_group, source_key)
                .await?,
            source_type,
        ))
    }

    async fn insert_atom(&self, draft: AtomDraft) -> Result<(), SqlxServiceError> {
        let uuid_repo = &self.uuid_repo;
        let mut tx = self.db_pool.begin().await.map_err(Transaction)?;

        let url = draft.url.as_str();
        let upsert_id = uuid_repo
            .upsert_uuid_mapping(&mut tx, SourceAtom, ExternalEntity::source_key(&draft))
            .await?;

        match upsert_id {
            UpsertMapping::Existing(_) => return Ok(()),
            UpsertMapping::New(id) => {
                query!(
                    r#"
                    INSERT INTO sources (id, source)
                    VALUES ($1, $2)
                    "#,
                    id,
                    SourceType::Atom
                )
                .execute(&mut *tx)
                .await
                .map_err(|error| Execute {
                    id: Some(id),
                    identifier: Some(url.to_string()),
                    error,
                })?;

                query!(
                    r#"
                    INSERT INTO source_atom_details (atom_details_id, url)
                    VALUES ($1, $2) ON CONFLICT(url) DO NOTHING
                    "#,
                    id,
                    url,
                )
                .execute(&mut *tx)
                .await
                .map_err(|error| Execute {
                    id: Some(id),
                    identifier: Some(url.to_string()),
                    error,
                })?;
            }
        };

        tx.commit().await.map_err(Transaction)?;

        Ok(())
    }

    async fn insert_telegram_channel(&self, draft: TelegramDraft) -> Result<(), SqlxServiceError> {
        let mut tx = self.db_pool.begin().await.map_err(Transaction)?;

        let upsert_id = self
            .uuid_repo
            .upsert_uuid_mapping(&mut tx, SourceTelegram, draft.username.as_str())
            .await?;

        match upsert_id {
            UpsertMapping::Existing(_) => return Ok(()),
            UpsertMapping::New(id) => {
                let username = draft.username;

                let id = query!(
                    r#"
                    INSERT INTO sources (id, source)
                    VALUES ($1, $2)
                    RETURNING id as "id: Uuid"
                    "#,
                    id,
                    SourceType::Telegram,
                )
                .fetch_one(&mut *tx)
                .await
                .map_err(|error| Execute {
                    id: Some(id),
                    identifier: Some(username.to_string()),
                    error,
                })?
                .id;

                query!(
                    r#"
                    INSERT INTO source_telegram_details (telegram_details_id, username)
                    VALUES ($1, $2)
                    "#,
                    id,
                    username,
                )
                .execute(&mut *tx)
                .await
                .map_err(|error| Execute {
                    id: Some(id),
                    identifier: Some(username.to_string()),
                    error,
                })?;
            }
        };

        tx.commit().await.map_err(Transaction)?;

        Ok(())
    }
}

fn query_reply_to_atom(query_result: SourceQueryResult) -> Result<AtomSource, InvalidArgument> {
    let atom_url = match query_result.atom_url {
        None => {
            return Err(InvalidArgument {
                name: "atom_url".to_string(),
                value: String::new(),
                reason: format!("Missing URL for Atom source {}", query_result.id),
            });
        }
        Some(url) => url.0,
    };

    Ok(AtomSource::new(
        query_result.id,
        query_result.created_at,
        atom_url,
    ))
}

fn query_reply_to_telegram(
    query_result: SourceQueryResult,
) -> Result<TelegramSource, InvalidArgument> {
    let source = TelegramSource::new(
        query_result.id,
        query_result.telegram_username.unwrap_or_default(),
        query_result.created_at,
    )?;

    Ok(source)
}

fn query_reply_to(query_result: SourceQueryResult) -> Result<SourceEnum, InvalidArgument> {
    Ok(match query_result.source {
        SourceType::Atom => query_reply_to_atom(query_result).map(SourceEnum::Atom)?,
        SourceType::Telegram => query_reply_to_telegram(query_result).map(SourceEnum::Telegram)?,
    })
}

#[async_trait]
impl SourceService for SqliteSourceService {
    async fn add(&self, draft: SourceDraft) -> Result<(), AddError> {
        Ok(match draft {
            SourceDraft::Atom(draft) => self.insert_atom(draft).await,
            SourceDraft::Telegram(draft) => self.insert_telegram_channel(draft).await,
        }
        .map_err(|e| AddError(Box::new(e)))?)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<SourceEnum, GetError> {
        let result = query_as!(
            SourceQueryResult,
            r#"
            SELECT
                src.id as "id: Uuid",
                src.source as "source: SourceType",
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
        .map_err(|e| GetError::Internal(Box::new(e)))?;

        match result {
            Some(source) => {
                let source = query_reply_to(source).map_err(|e| GetError::Internal(Box::new(e)))?;
                Ok(source)
            }
            None => Err(GetError::NotFound {
                id: id.to_string(),
                entity: String::from("source"),
            }),
        }
    }

    async fn get_all(&self) -> Result<impl IntoIterator<Item = SourceEnum>, GetAllError> {
        query_as!(
            SourceQueryResult,
            r#"
            SELECT
                src.id as "id: Uuid",
                src.source as "source: SourceType",
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
        .map_err(|error| GetAllError(Box::new(error)))?
        .into_iter()
        .map(|row| query_reply_to(row).map_err(|error| GetAllError(Box::new(error))))
        .collect::<Result<Vec<_>, _>>()
        .map(IntoIterator::into_iter)
    }

    async fn drop_by_draft(&self, draft: SourceDraft) -> Result<(), DropError> {
        let (id, source_type) = self
            .get_data_by_draft(draft)
            .await
            .map_err(|error| DropError(Box::new(error)))?;

        query!(
            r#"
            DELETE FROM uuid_mappings
            WHERE external_id = $1 AND internal_id IN (
                SELECT id
                FROM sources
                WHERE source = $2
            )
            "#,
            id,
            source_type,
        )
        .execute(&self.db_pool)
        .await
        .map_err(|error| DropError(Box::new(error)))?;

        Ok(())
    }
}
