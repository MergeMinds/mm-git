use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::Type, Clone, Copy, Serialize, Deserialize)]
#[sqlx(type_name = "acl_role", rename_all = "lowercase")]
pub enum AclRole {
    Read,
    Write,
}

#[derive(Serialize, Deserialize)]
pub struct Acl {
    pub repo_id: Vec<u8>,
    pub user_id: Uuid,
    pub role: AclRole,
}

#[derive(Serialize, Deserialize)]
pub struct AclRemove {
    pub repo_id: Vec<u8>,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct SshAuth {
    pub key_fingerprint: Vec<u8>,
    pub user_id: Uuid,
}

