use crate::utils::fs::get_db_uri;
use sqlx::SqlitePool;
use tokio::sync::OnceCell;
static DB_POOL: OnceCell<SqlitePool> = OnceCell::const_new();

pub type DBInitError = sqlx::Error;

pub async fn init_db_pool() -> Result<SqlitePool, DBInitError> {
    DB_POOL
        .get_or_try_init(|| async {
            let uri = get_db_uri();
            let pool = SqlitePool::connect(&uri).await?;
            Ok(pool)
        })
        .await
        .cloned()
}
