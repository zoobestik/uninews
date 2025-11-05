use crate::utils::fs::get_db_uri;
use sqlx::{Sqlite, SqlitePool, query};
use tokio::sync::OnceCell;
use uuid::Uuid;

static DB_POOL: OnceCell<SqlitePool> = OnceCell::const_new();

pub async fn init_db_pool() -> Result<SqlitePool, String> {
    DB_POOL
        .get_or_try_init(|| async {
            SqlitePool::connect(&get_db_uri()?)
                .await
                .map_err(|e| e.to_string())
        })
        .await
        .cloned()
}

pub async fn upsert_uuid_mapping<'c, E>(conn: E, source_id: &Uuid) -> Result<Uuid, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = Sqlite>,
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
    .await?
    .internal_id;

    Ok(id)
}
