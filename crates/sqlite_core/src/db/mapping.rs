use crate::db::errors::SqlxServiceError;
use sqlx::{Executor, Sqlite, query};
use uuid::Uuid;

pub async fn upsert_uuid_mapping<'c, E>(conn: E, source_id: &Uuid) -> Result<Uuid, SqlxServiceError>
where
    E: Executor<'c, Database = Sqlite>,
{
    let new_id = Uuid::now_v7();
    let id = query!(
        r#"
        INSERT INTO uuid_mappings (internal_id, external_id)
        VALUES ($1, $2)
        ON CONFLICT(external_id) DO UPDATE SET internal_id = internal_id
        RETURNING internal_id as "internal_id: Uuid"
        "#,
        new_id,
        source_id,
    )
    .fetch_one(conn)
    .await
    .map_err(|error| SqlxServiceError::Execute {
        id: Some(new_id),
        identifier: Some(source_id.to_string()),
        error,
    })?
    .internal_id;

    Ok(id)
}
