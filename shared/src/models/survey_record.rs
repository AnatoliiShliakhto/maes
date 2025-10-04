use crate::{models::*, utils::*};
use ::indexmap::IndexMap;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct SurveyRecord {
    pub id: String,
    pub workspace: String,
    pub name: String,
    pub node: String,
    pub path: String,
    pub total: usize,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub categories: IndexMap<String, SurveyRecordCategory>,
    pub metadata: Metadata,
}

impl SurveyRecord {
    pub fn to_base(&self) -> Self {
        Self {
            id: self.id.clone(),
            workspace: self.workspace.clone(),
            name: self.name.clone(),
            node: self.node.clone(),
            path: self.path.clone(),
            total: self.total,
            categories: Default::default(),
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct SurveyRecordCategory {
    pub id: String,
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub questions: IndexMap<String, SurveyCategoryItem>,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub answers: IndexMap<String, SurveyCategoryItem>,
    pub results: Grid<usize>,
}
