use crate::{prelude::*, pages::*, elements::*};

#[derive(Clone, PartialEq, Routable)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Route {
    #[layout(CleanLayout)]
    #[route("/login")]
    Login {},
    #[end_layout]
    #[layout(AppLayout)]
    #[redirect("/:.._segments", |_segments: Vec<String>| Route::Tasks {})]
    #[route("/tasks")]
    Tasks {},
    #[route("/tasks/wizard")]
    TaskWizard {},
    #[route("/workspace")]
    WorkspaceManager {},
    #[route("/workspace/quizzes")]
    WorkspaceQuizzes {},
    #[route("/workspace/surveys")]
    WorkspaceSurveys {},
    #[route("/settings")]
    Settings {},
    #[route("/about")]
    About {},
    #[route("/reports")]
    Reports {},
    #[route("/students")]
    Students {},
    #[route("/workspace/quizzes/:quiz_id")]
    QuizManager { quiz_id: String },
    #[route("/workspace/surveys/:survey_id")]
    SurveyManager { survey_id: String },
}