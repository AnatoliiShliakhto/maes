use crate::{models::*, utils::*};
use ::serde::{Deserialize, Serialize};
use ::std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Task {
    pub id: String,
    pub workspace: String,
    pub kind: EntityKind,
    pub name: String,
    pub node: String,
    pub path: String,
    pub progress: usize,
    pub metadata: Metadata,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Tasks {
    workspace: String,
    inner: HashMap<String, Task>,
}

impl Tasks {
    pub fn new(workspace: impl Into<String>) -> Self {
        Self {
            workspace: workspace.into(),
            inner: HashMap::<String, Task>::new(),
        }
    }
}

impl Deref for Tasks {
    type Target = HashMap<String, Task>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Tasks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Cachable for Tasks {
    fn kind() -> EntityKind {
        EntityKind::Tasks
    }

    fn get_id(&self) -> String {
        TASKS.to_string()
    }

    fn get_ws(&self) -> String {
        self.workspace.clone()
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TaskCategory {
    pub id: String,
    pub name: String,
    pub count: usize,
    pub total: usize,
}