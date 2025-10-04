use super::{editor_category::*, editor_survey::*};
use crate::{pages::*, prelude::*};

#[component]
pub fn SurveyEditor() -> Element {
    let survey = use_context::<Signal<Survey>>();
    let selected = use_context::<Signal<SurveyManagerAction>>();

    rsx! {
        match &*selected.read() {
            SurveyManagerAction::Survey => rsx! {
                SurveyEditorSurvey {
                    key: "{survey.read().id}",
                }
            },
            SurveyManagerAction::Category(category_id) => rsx! {
                SurveyEditorCategory {
                    key: "editor-{category_id}",
                    category_id: "{category_id}",
                }
            },
        }
    }
}
