use crate::{prelude::*, pages::{*, quiz::*, survey::*}, elements::*};

#[derive(Clone, PartialEq, Routable)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Route {
    #[layout(CleanLayout)]
    #[route("/quiz/take")]
    QuizTake {},
    #[end_layout]
    #[layout(ClientLayout)]
    
    #[route("/quiz/details/:workspace/:task/:student")]
    QuizDetails { workspace: String, task: String, student: String },
    #[route("/quiz/start/:workspace/:task/:student")]
    QuizStart { workspace: String, task: String, student: String },
    #[route("/quiz/finish")]
    QuizFinish {},

    #[route("/survey/details/:workspace/:task")]
    SurveyDetails { workspace: String, task: String },
    
    #[route("/:kind/:workspace/:task/:..segments")]
    Initial { kind: String, workspace: String, task: String, segments: Vec<String> },
    #[route("/error")]
    ErrorPage {},
    #[redirect("/:.._segments", |_segments: Vec<String>| Route::Home {})]
    #[route("/")]
    Home {},
}