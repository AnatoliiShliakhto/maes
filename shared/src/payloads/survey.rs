use crate::models::*;
use ::serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize)]
pub struct CreateSurveyPayload {
    pub name: String,
    pub node: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateSurveyPayload {
    pub name: String,
    pub node: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateSurveyCategoryPayload {
    pub name: String,
    pub order: usize,
    pub answers: Vec<SurveyCategoryItem>,
    pub questions: Vec<SurveyCategoryItem>,
}