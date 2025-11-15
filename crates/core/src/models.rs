pub mod news;
pub mod source;

pub trait ExternalEntity {
    fn source_key(&self) -> &str;
}
