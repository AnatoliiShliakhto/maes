use super::{child::*, mock::*};

#[derive(Clone, PartialEq)]
pub enum WindowKind {
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
        match &kind {
            WindowKind::Mock { url } => open_mock_window(title, url.clone()),
            WindowKind::WiFiInstruction => open_child_window(title, kind),
            WindowKind::QuizTickets { .. } |
            WindowKind::SurveyTickets { .. } => open_child_window(title, kind),
            _ => ()  // TODO 
        }
    }
}