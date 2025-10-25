use super::News;
use super::service::NewsService;
use async_trait::async_trait;
use sqlx::{SqlitePool, query};
use std::sync::Arc;
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
        let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;

        for news in news {
            let source_id = news.source_id();

            let uuid = query!(
                r#"
                SELECT internal_id as "internal_id: Uuid" FROM uuid_mappings
                WHERE external_id IN (?1)
                "#,
                source_id,
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            let new_id = Uuid::now_v7();
            let id = uuid.map_or(new_id, |uuid| uuid.internal_id);

            let parent_id = news.parent_id();
            let title = news.title();
            let description = news.description();
            let content = news.content();

            query!(
                r#"
                INSERT INTO articles (id, parent_id, title, description, content)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT(id) DO UPDATE SET
                    title = $3, description = $4, content = $5
                "#,
                id,
                parent_id,
                title,
                description,
                content,
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            if new_id != id {
                query!(
                    r#"
                        INSERT INTO uuid_mappings (internal_id, external_id)
                        VALUES ($1, $2)
                        "#,
                    id,
                    source_id,
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
