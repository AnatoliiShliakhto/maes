use crate::{models::*, utils::*};
use ::indexmap::IndexMap;
use ::serde::{Deserialize, Serialize};
use ::std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Student {
    pub id: String,
    pub node: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Students {
    workspace: String,
    inner: IndexMap<String, Student>,
}

impl Students {
    pub fn new(workspace: impl Into<String>) -> Self {
        Self {
            workspace: workspace.into(),
            inner: IndexMap::new(),
        }
    }
}

impl Deref for Students {
    type Target = IndexMap<String, Student>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Students {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Cachable for Students {
    fn kind() -> EntityKind {
        EntityKind::Students
    }

    fn get_id(&self) -> String {
        STUDENTS.to_string()
    }

    fn get_ws(&self) -> String {
        self.workspace.to_string()
    }
}
