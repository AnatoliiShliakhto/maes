use ::chrono::{TimeZone, Utc};
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Metadata {
    pub created_by: String,
    pub created_at: i64,
    pub updated_by: String,
    pub updated_at: i64,
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new("".to_string())
    }
}
impl Metadata {
    pub fn new(created_by: impl Into<String>) -> Self {
        let username = created_by.into();
        Self {
            created_by: username.clone(),
            created_at: Utc::now().timestamp(),
            updated_by: username,
            updated_at: Utc::now().timestamp(),
        }
    }

    pub fn update(&mut self, updated_by: impl Into<String>) -> &Self {
        self.updated_by = updated_by.into();
        self.updated_at = Utc::now().timestamp();
        self
    }

    pub fn created_at(&self) -> String {
        Utc.timestamp_opt(self.created_at, 0)
            .unwrap()
            .format("%d.%m.%Y")
            .to_string()
    }

    pub fn updated_at(&self) -> String {
        Utc.timestamp_opt(self.updated_at, 0)
            .unwrap()
            .format("%d.%m.%Y")
            .to_string()
    }
}
