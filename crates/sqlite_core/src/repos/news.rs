use crate::db::errors::SqlxServiceError;
use crate::db::errors::SqlxServiceError::{DBInit, Execute, Transaction};
use crate::db::init::{DBInitError, init_db_pool};
use crate::db::mapping::upsert_uuid_mapping;
use async_trait::async_trait;
use news_core::models::news::News;
use news_core::repos::news::NewsUpdateError::{Internal, UpdateItem};
use news_core::repos::news::{NewsRepository, NewsUpdateError};
use sqlx::{SqlitePool, query};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub struct SqliteNewsRepository {
    db_pool: SqlitePool,
}

impl SqliteNewsRepository {
    pub async fn new() -> Result<Self, DBInitError> {
        Ok(Self {
            db_pool: init_db_pool().await?,
        })
    }

    async fn update_news(&self, news: &[Arc<dyn News>]) -> Result<(), SqlxServiceError> {
        let mut tx = self.db_pool.begin().await.map_err(Transaction)?;

        let mut modified: HashMap<Uuid, usize> = HashMap::new();

        for news in news {
            let source_id = news.source_id();

            let id = upsert_uuid_mapping(&mut *tx, &source_id).await?;
            let parent_id = news.parent_id();
            let title = news.title();
            let description = news.description();
            let content = news.content();

            let result = query!(
                r#"
                INSERT INTO articles (id, parent_id, title, description, content)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT(id) DO UPDATE SET
                    parent_id = excluded.parent_id,
                    title = excluded.title,
                    description = excluded.description,
                    content = excluded.content,
                    updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                WHERE
                    articles.parent_id != excluded.parent_id OR
                    articles.title != excluded.title OR
                    articles.description != excluded.description OR
                    articles.content IS NOT excluded.content OR
                    (articles.content IS NULL AND excluded.content IS NOT NULL) OR
                    (articles.content IS NOT NULL AND excluded.content IS NULL)
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
            .map_err(|error| Execute {
                id: Some(id),
                identifier: Some(title.to_string()),
                error,
            })?;

            if let Some(record) = result {
                let parent_id = record.parent_id;
                let numbers = modified.get(&parent_id).unwrap_or(&0) + 1;
                modified.insert(parent_id, numbers);
            }
        }

        tx.commit().await.map_err(Transaction)?;

        if !modified.is_empty() {
            for (uuid, modified) in &modified {
                info!("[news_service={}] {} articles modified", uuid, modified);
            }
        }

        Ok(())
    }
}

#[async_trait]
impl NewsRepository for SqliteNewsRepository {
    async fn update(&self, news: &[Arc<dyn News>]) -> Result<(), NewsUpdateError> {
        self.update_news(news).await.map_err(|e| match e {
            Execute {
                id,
                identifier,
                error,
            } => UpdateItem {
                id: id.unwrap_or_else(Uuid::nil),
                title: identifier.unwrap_or_else(|| "Unknown title".to_string()),
                error: Box::new(error),
            },
            DBInit(e) | Transaction(e) => Internal(Box::new(e)),
        })
    }
}
