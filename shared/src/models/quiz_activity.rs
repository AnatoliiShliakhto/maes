use crate::utils::*;
use ::indexmap::IndexMap;
use ::serde::{Deserialize, Serialize};
use ::serde_repr::{Deserialize_repr, Serialize_repr};
use ::std::collections::HashSet;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuizActivityDetails {
    pub workspace: String,
    pub quiz: String,
    pub quiz_name: String,
    pub duration: i64,
    pub student: String,
    pub student_rank: Option<String>,
    pub student_name: String,
    pub grade: usize,
    pub score: usize,
    pub can_take: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuizActivity {
    pub workspace: String,
    pub task: String,
    pub quiz: String,
    pub duration: i64,
    pub student: String,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub questions: IndexMap<String, QuizActivityQuestion>,
}

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
pub enum QuizActivityQuestionKind {
    #[default]
    Single = 0,
    Multiple = 1,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuizActivityQuestion {
    pub id: String,
    pub category: String,
    pub kind: QuizActivityQuestionKind,
    pub name: String,
    pub img: bool,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub answers: IndexMap<String, QuizActivityAnswer>,
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub answered: HashSet<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuizActivityAnswer {
    pub id: String,
    pub name: String,
    pub img: bool,
}
