use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateWorkspacePayload {
    pub name: String,
    pub username: String,
    pub login: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateWorkspaceTreeNodePayload {
    pub id: String,
    pub name: String,
    pub parent: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateWorkspaceTreeNodePayload {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeleteWorkspaceTreeNodePayload {
    pub id: String,
}