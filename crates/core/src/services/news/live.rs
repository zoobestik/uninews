use super::News;
use super::service::NewsService;
use async_trait::async_trait;
use sqlx::{SqlitePool, query};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub struct LiveNewsService {
    db_pool: SqlitePool,
}

impl LiveNewsService {
    #[must_use]
    pub const fn new(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl NewsService for LiveNewsService {
    async fn update_news(&self, news: &[Arc<dyn News>]) -> Result<(), String> {
        let mut modified: usize = 0;
        let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;

        for news in news {
            let id = Uuid::now_v7();
            let source_id = news.source_id();

            let result = query!(
                r#"
                    INSERT INTO uuid_mappings (internal_id, external_id)
                    VALUES ($1, $2) ON CONFLICT(external_id) DO UPDATE SET
                       external_id = $2
                    RETURNING internal_id
                    "#,
                id,
                source_id,
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            let id = result.internal_id;

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
                    updated_at = strftime('%Y-m-%dT%H:%M:%fZ', 'now')
                WHERE
                    articles.title IS NOT excluded.title OR
                    articles.description IS NOT excluded.description OR
                    articles.content IS NOT excluded.content
                RETURNING
                    id as "id: Uuid"
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

            if result.is_some() {
                modified += 1;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        if modified != 0 {
            info!("[news_service] {0} articles modified", modified);
        }

        Ok(())
    }
}
