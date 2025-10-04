use crate::model::{Metadata, TreeNode};
use ::indexmap::IndexMap;
use ::serde::{Deserialize, Serialize};
use ::serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "users_as_vec"
    )]
    pub users: IndexMap<String, WorkspaceUser>,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "tree_as_vec"
    )]
    pub unit_tree: IndexMap<String, TreeNode>,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "tree_as_vec"
    )]
    pub quiz_tree: IndexMap<String, TreeNode>,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "tree_as_vec"
    )]
    pub survey_tree: IndexMap<String, TreeNode>,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "tree_as_vec"
    )]
    pub checklist_tree: IndexMap<String, TreeNode>,
    pub metadata: Metadata,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct WorkspaceUser {
    pub id: String,
    pub username: String,
    pub login: String,
    pub password: String,
    #[serde(default)]
    pub unit: String,
    pub role: WorkspaceRole,
}

#[repr(i32)]
#[derive(Clone, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum WorkspaceRole {
    Admin = 0,
    User = 1,
    Supervisor = 2,
}

mod users_as_vec {
    use super::WorkspaceUser;
    use ::indexmap::IndexMap;
    use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(
        map: &IndexMap<String, WorkspaceUser>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        map.values()
            .collect::<Vec<&WorkspaceUser>>()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<IndexMap<String, WorkspaceUser>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<WorkspaceUser>::deserialize(deserializer)?;
        // vec.sort_by(|a, b| a.username.cmp(&b.username));
        let mut map = IndexMap::with_capacity(vec.len());
        for item in vec {
            map.insert(item.id.clone(), item);
        }
        Ok(map)
    }
}

mod tree_as_vec {
    use super::TreeNode;
    use ::indexmap::IndexMap;
    use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(map: &IndexMap<String, TreeNode>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        map.values()
            .collect::<Vec<&TreeNode>>()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<IndexMap<String, TreeNode>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<TreeNode>::deserialize(deserializer)?;
        // vec.sort_by(|a, b| a.name.cmp(&b.name));
        let mut map = IndexMap::with_capacity(vec.len());
        for item in vec {
            map.insert(item.id.clone(), item);
        }
        Ok(map)
    }
}
