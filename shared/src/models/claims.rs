use crate::models::WorkspaceRole;
use ::serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub ws_id: String,
    pub username: String,
    pub workspace: String,
    pub version: String,
    pub node: String,
    pub role: WorkspaceRole,
    pub token: String,
}

impl Claims {
    pub fn is_authorized(&self) -> bool {
        self.role != WorkspaceRole::Unauthorized
    }
    
    pub fn is_admin(&self) -> bool {
        self.role == WorkspaceRole::Admin
    }
    pub fn is_supervisor(&self) -> bool {
        self.role == WorkspaceRole::Supervisor || self.role == WorkspaceRole::Admin
    }
    pub fn is_user(&self) -> bool {
        self.role == WorkspaceRole::User
    }
}