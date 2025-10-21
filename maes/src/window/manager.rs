use crate::services::*;
use super::{child::*, mock::*};

#[derive(Clone, PartialEq)]
pub enum WindowKind {
    About,
    Mock { url: String },
    WiFiInstruction,
    QuizTickets { task: String },
    QuizReport { entity: String },
    SurveyTickets { task: String },
    SurveyReport { entity: String },
}

#[derive(Copy, Clone)]
pub struct WindowManager;

impl WindowManager {
    pub fn open_window(title: impl Into<String>, kind: WindowKind) {
        let claims = AuthService::claims();
        match &kind {
            WindowKind::About => open_child_window(title, kind, claims),
            WindowKind::Mock { url } => open_mock_window(title, url.clone()),
            WindowKind::WiFiInstruction |
            WindowKind::QuizTickets { .. } |
            WindowKind::SurveyTickets { .. } |
            WindowKind::QuizReport { .. } |
            WindowKind::SurveyReport { .. } => open_child_window(title, kind, claims),
        }
    }
}