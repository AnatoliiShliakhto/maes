use crate::model::WorkspaceRole;
use ::serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub username: String,
    pub workspace: String,
    pub workspace_name: String,
    pub unit: String,
    pub role: WorkspaceRole,
    pub token: String,
}
