use crate::{models::*, utils::*};
use ::indexmap::IndexMap;
use ::serde::{Deserialize, Serialize};
use ::std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuizRecord {
    pub id: String,
    pub workspace: String,
    pub name: String,
    pub node: String,
    pub path: String,
    pub duration: i64,
    pub grade: QuizGrade,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub categories: IndexMap<String, QuizRecordCategory>,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub students: IndexMap<String, QuizRecordStudent>,
    pub answers: Grid<HashMap<String, HashSet<String>>>,
    pub results: Grid<u8>,
    pub metadata: Metadata,
}

impl QuizRecord {
    pub fn to_base(&self) -> Self {
        QuizRecord {
            id: self.id.clone(),
            workspace: self.workspace.clone(),
            name: self.name.clone(),
            path: self.path.clone(),
            duration: self.duration,
            students: self.students.clone(),
            results: self.results.clone(),
            metadata: self.metadata.clone(),
            ..Default::default()
        }
    }

    pub fn to_entity(&self) -> Entity {
        Entity {
            id: self.id.clone(),
            name: self.name.clone(),
            kind: EntityKind::QuizRecord,
            node: self.node.clone(),
            path: self.path.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuizRecordCategory {
    pub id: String,
    pub name: String,
    pub count: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuizRecordStudent {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<String>,
    pub name: String,
    pub attempts: usize,
    pub grade: u8,
}
