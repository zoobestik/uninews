use crate::utils::fs::get_db_uri;
use sqlx::SqlitePool;
use tokio::sync::OnceCell;

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
