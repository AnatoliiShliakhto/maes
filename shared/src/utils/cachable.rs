use crate::models::*;

pub trait Cachable: Send + Sync {
    fn kind() -> EntityKind;
    fn get_id(&self) -> String;
    fn get_ws(&self) -> String;
}

impl Cachable for Workspace {
    fn kind() -> EntityKind {
        EntityKind::Workspace
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_ws(&self) -> String {
        self.id.clone()
    }
}

impl Cachable for Quiz {
    fn kind() -> EntityKind {
        EntityKind::Quiz
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_ws(&self) -> String {
        self.workspace.clone()
    }
}

impl Cachable for Survey {
    fn kind() -> EntityKind {
        EntityKind::Survey
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_ws(&self) -> String {
        self.workspace.clone()
    }
}

impl Cachable for QuizRecord {
    fn kind() -> EntityKind {
        EntityKind::QuizRecord
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_ws(&self) -> String {
        self.workspace.clone()
    }
}

impl Cachable for SurveyRecord {
    fn kind() -> EntityKind {
        EntityKind::SurveyRecord
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_ws(&self) -> String {
        self.workspace.clone()
    }
}