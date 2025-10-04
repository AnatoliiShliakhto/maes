use super::{quiz_inspector::*, survey_inspector::*};
use crate::prelude::*;

#[component]
pub fn TaskInspector() -> Element {
    let kind = use_context::<Signal<EntityKind>>();
    let selected = use_context::<Signal<SelectedItem>>();

    rsx! {
        match *kind.read() {
            EntityKind::QuizRecord => rsx! { QuizInspector { key: "inspector-{selected.read().id}" } },
            EntityKind::SurveyRecord => rsx! { SurveyInspector { key: "inspector-{selected.read().id}" } },
            _ => rsx! {},
        }
    }
}
