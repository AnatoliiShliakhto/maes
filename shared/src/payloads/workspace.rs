use crate::models::WorkspaceRole;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateWorkspacePayload {
    pub name: String,
    pub username: String,
    pub login: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateWorkspaceTreeNodePayload {
    pub name: String,
    pub parent: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateWorkspaceTreeNodePayload {
    pub name: String,
    pub parent: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateWorkspaceUserPayload {
    pub username: String,
    pub login: String,
    pub password: String,
    pub node: String,
    pub role: WorkspaceRole,
}