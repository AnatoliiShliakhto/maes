use crate::model::Metadata;
use ::serde::{Deserialize, Serialize};
use ::serde_repr::{Deserialize_repr, Serialize_repr};
use ::std::{fmt, str::FromStr};

#[repr(i32)]
#[derive(Clone, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum EntityKind {
    Workspace = 0,

    Quizz = 1,
    Survey = 2,
    Checklist = 3,

    QuizTask = 100,
    SurveyTask = 101,
    ChecklistTask = 102,

    QuizArchive = 1000,
    SurveyArchive = 1001,
    ChecklistArchive = 1002,
}

#[derive(Debug, Clone)]
pub struct ParseEntityKindError;

impl fmt::Display for ParseEntityKindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid entity kind")
    }
}

impl std::error::Error for ParseEntityKindError {}

impl FromStr for EntityKind {
    type Err = ParseEntityKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = s.trim().to_ascii_lowercase();

        Ok(match key.as_str() {
            "workspace" => EntityKind::Workspace,
            "quizz" => EntityKind::Quizz,
            "survey" => EntityKind::Survey,
            "checklist" => EntityKind::Checklist,
            "quiz-task" => EntityKind::QuizTask,
            "survey-task" => EntityKind::SurveyTask,
            "checklist-task" => EntityKind::ChecklistTask,
            _ => return Err(ParseEntityKindError),
        })
    }
}

impl TryFrom<&str> for EntityKind {
    type Error = ParseEntityKindError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<String> for EntityKind {
    type Error = ParseEntityKindError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.as_str().parse()
    }
}

impl EntityKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityKind::Workspace => "workspace",
            EntityKind::Quizz => "quizz",
            EntityKind::Survey => "survey",
            EntityKind::Checklist => "checklist",
            EntityKind::QuizTask => "quiz-task",
            EntityKind::SurveyTask => "survey-task",
            EntityKind::ChecklistTask => "checklist-task",
            EntityKind::QuizArchive => "quiz-archive",
            EntityKind::SurveyArchive => "survey-archive",
            EntityKind::ChecklistArchive => "checklist-archive",
        }
    }
}

impl fmt::Display for EntityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub kind: EntityKind,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub workspace: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub section: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub progress: usize,
    pub metadata: Metadata,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            kind: EntityKind::Workspace,
            workspace: "".to_string(),
            section: "".to_string(),
            path: "".to_string(),
            progress: 0,
            metadata: Default::default(),
        }
    }
}

fn is_zero(v: &usize) -> bool {
    *v == 0
}
