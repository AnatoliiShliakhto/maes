use crate::{models::*, utils::*};
use ::indexmap::IndexMap;
use ::serde::{Deserialize, Serialize};
use ::serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub users: IndexMap<String, WorkspaceUser>,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub unit_tree: IndexMap<String, TreeNode>,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub quiz_tree: IndexMap<String, TreeNode>,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub survey_tree: IndexMap<String, TreeNode>,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub checklist_tree: IndexMap<String, TreeNode>,
    pub metadata: Metadata,
}

impl Workspace {
    pub fn to_entity(&self) -> Entity {
        Entity {
            id: self.id.clone(),
            name: self.name.clone(),
            kind: EntityKind::Workspace,
            metadata: self.metadata.clone(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WorkspaceUser {
    pub id: String,
    pub username: String,
    pub login: String,
    pub password: String,
    #[serde(default)]
    pub node: String,
    #[serde(default)]
    pub path: String,
    pub role: WorkspaceRole,
}

#[repr(i32)]
#[derive(Debug, Default, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum WorkspaceRole {
    #[default]
    Unauthorized = 0,
    User = 1,
    Supervisor = 2,
    Admin = 3,
}

impl From<String> for WorkspaceRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "admin" => WorkspaceRole::Admin,
            "user" => WorkspaceRole::User,
            "supervisor" => WorkspaceRole::Supervisor,
            _ => WorkspaceRole::User,
        }
    }
}


#[derive(Clone, Serialize, Deserialize)]
pub struct WorkspaceMetadata {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub version: i64,
}