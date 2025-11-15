use crate::utils::parse::parse_url;
use sqlx::error::BoxDynError;
use sqlx::sqlite::SqliteValueRef;
use sqlx::{Decode, Sqlite};
use std::ops::Deref;
use url::Url as UrlLib;

#[derive(Debug, Clone)]
pub struct Url(pub UrlLib);

impl Deref for Url {
    type Target = url::Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'r> Decode<'r, Sqlite> for Url {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let url_str = <String as Decode<Sqlite>>::decode(value)?;
        let inner_url = parse_url(&url_str)?;
        Ok(Self(inner_url))
    }
}
