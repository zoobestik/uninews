mod item;

use crate::source::atom::item::try_atom_news_from_rss_item;
use crate::state::AppState;
use futures::future::try_join_all;
use rss::Channel;
use std::io::Cursor;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info};
use uninews_core::models::atom::AtomSource;
use uninews_core::services::news::{News, NewsService};

async fn extract_items(
    source: &AtomSource,
    channel: Channel,
) -> Result<Vec<Arc<dyn News>>, String> {
    let news_futures: Vec<_> = channel
        .items
        .into_iter()
        .map(async |item| -> Result<Arc<dyn News>, String> {
            let news_item = try_atom_news_from_rss_item(source, item).await?;
            Ok(Arc::new(news_item) as Arc<dyn News>)
        })
        .collect();

    try_join_all(news_futures).await
}

async fn read_periodically(
    app_state: Arc<AppState>,
    source: &AtomSource,
) -> Result<Vec<Arc<dyn News>>, String> {
    let resp = app_state
        .http()
        .await?
        .request_by_schedule(&source.url)
        .await
        .map_err(|e| format!("Failed to fetch feed: {e}"))?;

    let content = resp
        .text()
        .await
        .map_err(|e| format!("Failed to get response text: {e}"))?;

    let news = extract_items(
        source,
        Channel::read_from(Cursor::new(&content))
            .map_err(|e| format!("Failed to parse feed content: {e}"))?,
    )
    .await?;

    app_state
        .storage()
        .await?
        .save_raw(source.id, content.as_ref())
        .await;

    Ok(news)
}

// @todo: In the `watch_updates` function, there seems to be a potential issue with an infinite loop (`loop`) without any delay or condition. This can cause the service to block resources at high priority and lead to performance degradation. If the loop fails, it could also leave the application in an unstable state.
pub async fn watch_atom_feed(app_state: Arc<AppState>, source: &AtomSource) -> Result<(), String> {
    info!("Watch [atom_feed=\"{0}\"] news", source.url);

    let news_service = app_state.news().await?;

    loop {
        let result = match read_periodically(app_state.clone(), source).await {
            Ok(news) => update_atom_news(source, news_service, news).await,
            Err(e) => Err(e),
        };

        if let Err(e) = result {
            error!("[atom_feed=\"{0}\"] {e}", source.url);
        }

        sleep(Duration::from_secs(60)).await;
    }
}

async fn update_atom_news(
    source: &AtomSource,
    news_service: &Arc<dyn NewsService>,
    news: Vec<Arc<dyn News>>,
) -> Result<(), String> {
    let len = news.len();
    let news: Vec<&dyn News> = news.iter().map(AsRef::as_ref).collect();
    news_service.update_news(news).await;
    debug!(
        "[atom_feed=\"{0}\"] Updated {1} news items",
        source.url, len
    );
    Ok(())
}
