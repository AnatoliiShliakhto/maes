use crate::prelude::*;
use ::std::str::FromStr;

#[component]
pub fn Initial(kind: String, workspace: String, task: String, segments: Vec<String>) -> Element {
    let navigator = use_navigator();

    let Ok(kind) = EntityKind::from_str(&kind) else {
        navigator.push(Route::Home {});
        return rsx! {};
    };

    match kind {
        EntityKind::QuizRecord => {
            if segments.len() != 1 {
                navigator.push(Route::Home {});
                return rsx! {};
            }
            navigator.push(Route::QuizDetails {
                workspace,
                task,
                student: segments[0].clone(),
            });
        }
        EntityKind::SurveyRecord => {
            navigator.push(Route::SurveyDetails { workspace, task });
        }
        _ => { navigator.push(Route::Home {}); }
    }

    rsx! {}
}
