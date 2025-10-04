use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddStudentPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<String>,
    pub name: String,
}