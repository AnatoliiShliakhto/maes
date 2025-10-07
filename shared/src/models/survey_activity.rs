use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct SurveyActivityDetails {
    pub workspace: String,
    pub survey: String,
    pub survey_name: String,
}