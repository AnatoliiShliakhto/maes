use ::serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthPayload {
    pub workspace: String,
    pub login: String,
    pub password: String,
}