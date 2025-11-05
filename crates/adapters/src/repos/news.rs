use crate::utils::sqlite::tools::upsert_uuid_mapping;
use async_trait::async_trait;
use sqlx::{SqlitePool, query};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;
use uninews_core::models::News;
use uninews_core::repos::news::NewsRepository;
use uuid::Uuid;

pub struct SqliteNewsRepository {
    db_pool: SqlitePool,
}

impl SqliteNewsRepository {
    #[must_use]
    pub const fn new(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl NewsRepository for SqliteNewsRepository {
    async fn update_news(&self, news: &[Arc<dyn News>]) -> Result<(), String> {
        let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;

        let mut modified: HashMap<Uuid, usize> = HashMap::new();

        for news in news {
            let source_id = news.source_id();

            let id = upsert_uuid_mapping(&mut *tx, &source_id)
                .await
                .map_err(|e| e.to_string())?;

            let parent_id = news.parent_id();
            let title = news.title();
            let description = news.description();
            let content = news.content();

            let result = query!(
                r#"
                INSERT INTO articles (id, parent_id, title, description, content)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT(id) DO UPDATE SET
                    title = excluded.title,
                    description = excluded.description,
                    content = excluded.content,
                    updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                WHERE
                    articles.title IS NOT excluded.title OR
                    articles.description IS NOT excluded.description OR
                    articles.content IS NOT excluded.content
                RETURNING
                    parent_id as "parent_id: Uuid"
                "#,
                id,
                parent_id,
                title,
                description,
                content,
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            if let Some(record) = result {
                let parent_id = record.parent_id;
                let numbers = modified.get(&parent_id).unwrap_or(&0) + 1;
                modified.insert(parent_id, numbers);
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        if !modified.is_empty() {
            for (uuid, modified) in &modified {
                info!("[news_service={}] {} articles modified", uuid, modified);
            }
        }

        Ok(())
    }
}
