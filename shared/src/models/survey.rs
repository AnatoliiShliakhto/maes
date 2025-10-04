use crate::{models::*, utils::*};
use ::indexmap::IndexMap;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Survey {
    pub id: String,
    pub workspace: String,
    pub node: String,
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "indexmap_as_vec"
    )]
    pub categories: IndexMap<String, SurveyCategory>,
    pub metadata: Metadata,
}

impl Survey {
    pub fn to_base(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            workspace: self.workspace.clone(),
            node: self.node.clone(),
            categories: Default::default(),
            metadata: self.metadata.clone(),
        }
    }
    
    pub fn to_entity(&self) -> Entity {
        Entity {
            id: self.id.clone(),
            name: self.name.clone(),
            kind: EntityKind::Survey,
            node: self.node.clone(),
            metadata: self.metadata.clone(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct SurveyCategory {
    pub id: String,
    pub name: String,
    pub order: usize,
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
}

impl SurveyCategory {
    pub fn to_base(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            order: self.order,
            questions: Default::default(),
            answers: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct SurveyCategoryItem {
    pub id: String,
    pub name: String,
}