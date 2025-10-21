use crate::{components::inputs::*, prelude::*, services::*};

#[component]
pub fn SurveyEditorSurvey() -> Element {
    let claims = AuthService::claims();
    let mut survey = use_context::<Signal<Survey>>();
    let survey_guard = survey.read();

    let save_action = move |evt: FormEvent| {
        evt.stop();
        let survey_guard = survey.read();
        let Some(name) = form_values!(evt, "name") else {
            ToastService::error(t!("missing-fields"));
            return;
        };
        api_fetch!(
            PATCH,
            format!("/api/v1/manager/surveys/{survey_id}", survey_id = survey_guard.id),
            UpdateSurveyPayload {
                name,
                node: survey_guard.node.clone(),
                categories: vec![],
            },
            on_success = move |body: Survey| {
                survey.with_mut(|s| {
                    s.name = body.name;
                    s.node = body.node;
                });
                ToastService::success(t!("saved"))
            },
        )
    };

    rsx! {
        div {
            class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 items-center h-10",
            i { class: "bi bi-three-dots-vertical" }
            div {
                class: "w-full",
                { t!("survey") }
            }
            if claims.is_admin() {
                ul {
                    class: "menu menu-horizontal p-0 m-0 text-base-content",
                    li {
                        button {
                            class: "hover:text-success",
                            form: "form-survey-edit",
                            i { class: "bi bi-floppy" }
                            { t!("save") }
                        }
                    }
                }
            }
        }
        div {
            class: "h-0.25 bg-base-300 mx-4 my-1",
        }
        form {
            class: "flex-scrollable gap-4 px-3 my-2",
            id: "form-survey-edit",
            autocomplete: "off",
            onsubmit: move |evt| {
                if claims.is_admin() {
                    save_action(evt)
                } else {
                    evt.prevent_default()
                }
            },
            input {
                r#type: "submit",
                style: "position: absolute; left: -9999px; width: 1px; height: 1px;",
                tabindex: -1,
            }

            fieldset {
                class: "fieldset p-2",
                legend {
                    class: "fieldset-legend text-sm text-primary",
                    i { class: "bi bi-wrench-adjustable-circle" }
                    { t!("survey-settings") }
                }
                TextArea {
                    class: "min-h-10",
                    name: "name",
                    required: true,
                    minlength: 3,
                    maxlength: 100,
                    placeholder: t!("survey-placeholder"),
                    initial_value: "{survey_guard.name}",
                }
            }
        }
    }
}
