use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuizActivityDetails {
    pub workspace: String,
    pub quiz: String,
    pub quiz_name: String,
    pub duration: i64,
    pub student: String,
    pub student_rank: Option<String>,
    pub student_name: String,
    pub grade: u8,
    pub score: u8,
    pub can_take: bool,
}