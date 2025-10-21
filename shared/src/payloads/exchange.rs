use ::serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ExchangeExportPayload {
    pub entities: Vec<String>,
    pub path: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ExchangeImportPayload {
    pub path: String,
}