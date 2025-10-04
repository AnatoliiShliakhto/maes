use crate::models::*;

pub trait HasId<K> {
    fn key(&self) -> K;
}

impl HasId<String> for WorkspaceUser {
    fn key(&self) -> String { self.id.clone() }
}

impl HasId<String> for TreeNode {
    fn key(&self) -> String { self.id.clone() }
}

impl HasId<String> for QuizCategory {
    fn key(&self) -> String { self.id.clone() }
}

impl HasId<String> for QuizQuestion {
    fn key(&self) -> String { self.id.clone() }
}

impl HasId<String> for QuizAnswer {
    fn key(&self) -> String { self.id.clone() }
}

impl HasId<String> for SurveyCategory {
    fn key(&self) -> String { self.id.clone() }
}

impl HasId<String> for SurveyCategoryItem {
    fn key(&self) -> String { self.id.clone() }
}

impl HasId<String> for QuizRecordCategory {
    fn key(&self) -> String { self.id.clone() }
}

impl HasId<String> for QuizRecordStudent {
    fn key(&self) -> String { self.id.clone() }
}

impl HasId<String> for SurveyRecordCategory {
    fn key(&self) -> String { self.id.clone() }
}

pub mod indexmap_as_vec {
    use super::HasId;
    use ::indexmap::IndexMap;
    use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, K, V>(
        map: &IndexMap<K, V>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        V: Serialize,
    {
        map.values().collect::<Vec<&V>>().serialize(serializer)
    }

    pub fn deserialize<'de, D, K, V>(deserializer: D) -> Result<IndexMap<K, V>, D::Error>
    where
        D: Deserializer<'de>,
        V: Deserialize<'de> + HasId<K>,
        K: std::hash::Hash + Eq,
    {
        let vec = Vec::<V>::deserialize(deserializer)?;
        let mut map = IndexMap::with_capacity(vec.len());
        for item in vec {
            map.insert(item.key(), item);
        }
        Ok(map)
    }
}