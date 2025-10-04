use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct TreeNode {
    pub id: String,
    #[serde(default)]
    pub parent: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<String>,
}

#[derive(Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct SelectedItem {
    pub id: String,
    pub name: String,
    pub path: String,
}

impl SelectedItem {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            path: "".to_string(),
        }
    }
}