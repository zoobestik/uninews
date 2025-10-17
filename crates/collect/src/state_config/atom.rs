use crate::services::AppServices;
use crate::source::atom::Atom;
use serde::Deserialize;
use std::sync::Arc;
use tokio::try_join;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawAtom {
    source_url: String,
}

impl RawAtom {
    pub fn source_url(&self) -> &str {
        &self.source_url
    }
}

pub async fn try_atom_from_raw(
    item: &RawAtom,
    app_state: Arc<AppServices>,
) -> Result<Atom, String> {
    let (http_service, news_service) =
        try_join!(app_state.http_service(), app_state.news_service())
            .map_err(|e| format!("Failed to get services: {e}"))?;

    Atom::try_new(&item.source_url, http_service, news_service)
}
