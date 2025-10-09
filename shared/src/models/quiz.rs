use crate::{models::*, utils::*};
use ::indexmap::IndexMap;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Quiz {
    pub id: String,
    pub name: String,
    pub workspace: String,
    pub node: String,
    pub attempts: usize,
    pub duration: i64,
    pub grade: QuizGrade,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub categories: IndexMap<String, QuizCategory>,
    pub metadata: Metadata,
}

impl Quiz {
    pub fn to_base(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            workspace: self.workspace.clone(),
            node: self.node.clone(),
            attempts: self.attempts,
            duration: self.duration,
            grade: self.grade.clone(),
            categories: Default::default(),
            metadata: self.metadata.clone(),
        }
    }

    pub fn to_entity(&self) -> Entity {
        Entity {
            id: self.id.clone(),
            name: self.name.clone(),
            kind: EntityKind::Quiz,
            node: self.node.clone(),
            metadata: self.metadata.clone(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuizGrade {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Default for QuizGrade {
    fn default() -> Self {
        Self {
            a: 75,
            b: 50,
            c: 25,
        }
    }
}

impl QuizGrade {
    pub fn calc(&self, score: usize) -> usize {
        match score {
            s if s >= self.a => 5,
            s if s >= self.b => 4,
            s if s >= self.c => 3,
            _ => 2,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuizCategory {
    pub id: String,
    pub name: String,
    pub important: bool,
    pub count : usize,
    pub order: usize,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub questions: IndexMap<String, QuizQuestion>,
}

impl QuizCategory {
    pub fn to_base(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            important: self.important,
            count: self.count,
            order: self.order,
            questions: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuizQuestion {
    pub id: String,
    pub name: String,
    pub img: bool,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub answers: IndexMap<String, QuizAnswer>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuizAnswer {
    pub id: String,
    pub name: String,
    pub img: bool,
    pub correct: bool,
}