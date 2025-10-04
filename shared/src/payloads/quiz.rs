use crate::models::*;
use ::serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize)]
pub struct CreateQuizPayload {
    pub name: String,
    pub node: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateQuizPayload {
    pub name: String,
    pub node: String,
    pub attempts: usize,
    pub duration: i64,
    pub grade: QuizGrade,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<QuizCategory>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateQuizCategoryPayload {
    pub name: String,
    pub important: bool,
    pub count: usize,
    pub order: usize,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateQuizQuestionPayload {
    pub name: String,
    pub answers: Vec<QuizAnswer>,
}