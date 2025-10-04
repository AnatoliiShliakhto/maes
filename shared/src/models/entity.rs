use crate::{models::*, utils::*};
use ::serde::{Deserialize, Serialize};
use ::serde_repr::{Deserialize_repr, Serialize_repr};
use ::std::{collections::HashMap, fmt, str::FromStr, ops::{Deref, DerefMut}};

pub const WORKSPACE: &str = "workspace";
pub const ENTITIES: &str = "entities";
pub const STUDENTS: &str = "students";
pub const TASKS: &str = "tasks";

#[repr(i32)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
pub enum EntityKind {
    #[default]
    Workspace = 0,
    Entities = 1,
    Students = 2,
    Tasks = 3,

    Quiz = 100,
    QuizRecord = 101,

    Survey = 110,
    SurveyRecord = 111,

    Checklist = 120,
    ChecklistRecord = 121,

    Json = 130,
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
            "entities" => EntityKind::Entities,
            "tasks" => EntityKind::Tasks,
            "students" => EntityKind::Students,
            "quiz" => EntityKind::Quiz,
            "survey" => EntityKind::Survey,
            "checklist" => EntityKind::Checklist,
            "quiz-record" => EntityKind::QuizRecord,
            "survey-record" => EntityKind::SurveyRecord,
            "checklist-record" => EntityKind::ChecklistRecord,
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
            EntityKind::Entities => "entities",
            EntityKind::Tasks => "tasks",
            EntityKind::Students => "students",

            EntityKind::Quiz => "quiz",
            EntityKind::Survey => "survey",
            EntityKind::Checklist => "checklist",

            EntityKind::QuizRecord => "quiz-record",
            EntityKind::SurveyRecord => "survey-record",
            EntityKind::ChecklistRecord => "checklist-record",

            EntityKind::Json => "json",
        }
    }
}

impl fmt::Display for EntityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub kind: EntityKind,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub node: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    pub metadata: Metadata,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            kind: EntityKind::Workspace,
            node: "".to_string(),
            path: "".to_string(),
            metadata: Default::default(),
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities {
    workspace: String,
    inner: HashMap<String, Entity>,
}

impl Entities {
    pub fn new(workspace: Entity) -> Self {
        let ws_id = workspace.id.clone();
        let mut entities = HashMap::<String, Entity>::new();
        entities.insert(workspace.id.clone(), workspace);
        Self {
            workspace: ws_id,
            inner: entities,
        }
    }
}

impl Deref for Entities {
    type Target = HashMap<String, Entity>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Entities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Cachable for Entities {
    fn kind() -> EntityKind {
        EntityKind::Entities
    }

    fn get_id(&self) -> String {
        ENTITIES.to_string()
    }

    fn get_ws(&self) -> String {
        self.workspace.clone()
    }
}

// fn is_zero(v: &usize) -> bool {
//     *v == 0
// }
