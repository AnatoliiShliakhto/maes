use crate::model::Metadata;
use ::serde::{Deserialize, Serialize};
use ::indexmap::IndexMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Survey {
    pub id: String,
    pub workspace: String,
    pub section: String,
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        with = "categories_as_vec"
    )]
    pub categories: IndexMap<String, SurveyCategory>,
    pub metadata: Metadata,
}

impl PartialEq for Survey {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Survey {
    pub fn get_categories(&self) -> Vec<&SurveyCategory> {
        self.categories.values().collect::<Vec<&SurveyCategory>>()
    }

    pub fn get_category(&self, id: impl Into<String>) -> Option<&SurveyCategory> {
        self.categories.get(&id.into())
    }

    pub fn upsert_category(&mut self, category: SurveyCategory) {
        self.categories.insert(category.id.clone(), category);
    }

    pub fn remove_category(&mut self, id: impl Into<String>) {
        self.categories.shift_remove(&id.into());
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SurveyCategory {
    pub id: String,
    pub name: String,
    pub order: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub questions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub answers: Vec<String>,
}

impl PartialEq for SurveyCategory {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SurveyCategoryItem {
    pub id: String,
    pub name: String,
}

mod categories_as_vec {
    use super::SurveyCategory;
    use ::serde::{Deserialize, Deserializer, Serialize, Serializer};
    use ::indexmap::IndexMap;

    pub fn serialize<S>(
        map: &IndexMap<String, SurveyCategory>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        map.values()
            .collect::<Vec<&SurveyCategory>>()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<IndexMap<String, SurveyCategory>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut vec = Vec::<SurveyCategory>::deserialize(deserializer)?;
        vec.sort_by(|a, b| a.order.cmp(&b.order).then(a.name.cmp(&b.name)));
        let mut map = IndexMap::with_capacity(vec.len());
        for item in vec {
            map.insert(item.id.clone(), item);
        }
        Ok(map)
    }
}
