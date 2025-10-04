use crate::{
    components::{dialogs::*, survey::*, widgets::*},
    prelude::*,
    services::*,
};

#[derive(Clone, PartialEq)]
pub enum SurveyManagerAction {
    Survey,
    Category(String),
}

#[component]
pub fn SurveyManager(survey_id: ReadOnlySignal<String>) -> Element {
    if !AuthService::claims().is_supervisor() {
        return rsx! {};
    }
    use_init_dialog();
    use_init_input_dialog();
    use_init_context_menu();

    use_context_provider(|| Signal::new(SurveyManagerAction::Survey));
    let mut survey = use_context_provider(|| Signal::new(Survey::default()));

    use_effect(move || {
        api_fetch!(
            GET,
            format!("/api/v1/manager/surveys/{id}", id = survey_id.read()),
            on_success = move |body: Survey| survey.set(body),
        )
    });

    rsx! {
        SplitPanel {
            key: "{survey_id}",
            left_title: t!("survey-navigator"),
            left: rsx! {
                div {
                    class: "flex-scrollable",
                    if !survey.read().id.is_empty() {
                        SurveyTree {}
                    }
                }
            },
            right: rsx! {
                div {
                    class: "flex-fixed",
                    if !survey.read().id.is_empty() {
                        SurveyEditor {}
                    }
                }
            }
        }
        DialogContainer { key: "survey-manager-dialog" }
        InputDialogContainer { key: "survey-manager-dialog" }
        ContextMenuContainer { key: "survey-manager-ctx-menu" }
    }
}
