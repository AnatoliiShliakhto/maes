use ::serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct AddImagePayload {
    pub path: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CopyImagesPayload {
    pub source_workspace: String,
    pub source_entity: String,
    pub destination_workspace: String,
    pub destination_entity: String,
}

