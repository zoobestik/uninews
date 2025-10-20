pub mod sqlite;

use crate::models::SourceType;
use uuid::Uuid;

// pub struct CreateSource {
//     r#type: SourceType,
// }


#[allow(dead_code)] // @todo: implement
pub trait SourceRepository {
    // async fn create(&self, draft: CreateSource) -> Result<SourceType, String>;
    async fn find_by_id(&self, id: Uuid) -> Result<SourceType, String>;
    async fn find_by_ext_id(&self, id: Uuid) -> Result<SourceType, String>;
}
