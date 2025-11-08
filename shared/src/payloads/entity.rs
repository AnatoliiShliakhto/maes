use ::serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateEntityPayload {
    pub name: String,
    pub path: String,
}