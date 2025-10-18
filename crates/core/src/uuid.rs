use uuid::Uuid;

#[must_use]
pub fn gen_consistent_uuid(group_id: &Uuid, key: &str) -> Uuid {
    Uuid::new_v5(group_id, key.as_bytes())
}
