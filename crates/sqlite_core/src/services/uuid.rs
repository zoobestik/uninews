use crate::db::errors::SqlxServiceError;
use crate::db::init::{DBInitError, init_db_pool};
use sqlx::{Sqlite, SqlitePool, Transaction, Type, query};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
pub enum UuidGroup {
    SourceTelegram,
    SourceAtom,
    News,
}

fn gen_consistent_id(parent_id: &Uuid, key: &str) -> Uuid {
    Uuid::new_v5(parent_id, key.as_bytes())
}

pub struct SqliteUuidService {
    db_pool: SqlitePool,
}

pub enum UpsertMapping {
    New(Uuid),
    Existing(Uuid),
}

impl SqliteUuidService {
    pub async fn init_lazy() -> Result<Self, DBInitError> {
        Ok(Self {
            db_pool: init_db_pool().await?,
        })
    }

    pub async fn get_by_source_key(
        &self,
        group: &UuidGroup,
        key: &str,
    ) -> Result<Uuid, SqlxServiceError> {
        let result = query!(
            r#"
            SELECT
                internal_id as "uuid: Uuid"
            FROM persistence_uuid
            WHERE group_type = ?1
            "#,
            group,
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| SqlxServiceError::Execute {
            id: None,
            identifier: Some(format!("{group:?}")),
            error: e,
        })?;

        Ok(gen_consistent_id(&result.uuid, key))
    }

    pub async fn upsert_uuid_mapping(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        group: UuidGroup,
        key: &str,
    ) -> Result<UpsertMapping, SqlxServiceError> {
        let parent_id = Uuid::now_v7();

        let uuid = query!(
            r#"
            INSERT INTO persistence_uuid (internal_id, group_type)
            VALUES (?1, ?2)
            ON CONFLICT(group_type) DO UPDATE SET internal_id = internal_id
            RETURNING internal_id as "uuid: Uuid"
            "#,
            parent_id,
            group,
        )
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| SqlxServiceError::Execute {
            id: None,
            identifier: Some(format!("{group:?}")),
            error: e,
        })?
        .uuid;

        let consistent_id = gen_consistent_id(&uuid, key);
        let internal_id = Uuid::now_v7();

        let id = query!(
            r#"
            INSERT INTO uuid_mappings (internal_id, external_id)
            VALUES ($1, $2)
            ON CONFLICT(external_id) DO UPDATE SET internal_id = internal_id
            RETURNING internal_id as "internal_id: Uuid"
            "#,
            internal_id,
            consistent_id,
        )
        .fetch_one(&mut **tx)
        .await
        .map_err(|error| SqlxServiceError::Execute {
            id: Some(internal_id),
            identifier: Some(consistent_id.to_string()),
            error,
        })?
        .internal_id;

        match id == internal_id {
            true => Ok(UpsertMapping::New(id)),
            false => Ok(UpsertMapping::Existing(id)),
        }
    }
}
