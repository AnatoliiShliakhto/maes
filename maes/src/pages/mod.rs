mod tasks;
mod login;
mod workspace_manager;
mod workspace_quizzes;
mod workspace_surveys;
mod settings;
mod about;
mod reports;
mod students;
mod quiz_manager;
mod survey_manager;
mod task_wizard;

pub use self::{
    tasks::*,
    workspace_manager::*,
    workspace_quizzes::*,
    workspace_surveys::*,
    login::*,   
    settings::*,
    about::*,
    reports::*,
    students::*,
    quiz_manager::*,
    survey_manager::*,
    task_wizard::*,   
};