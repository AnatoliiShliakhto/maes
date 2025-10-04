use crate::models::*;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CreateTaskPayload {
    pub id: String,
    pub node: String,
    pub name: String,
    pub path: String,
    pub categories: Vec<TaskCategory>,
}
