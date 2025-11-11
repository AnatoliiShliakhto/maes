use super::*;
use crate::{prelude::*, services::*, utils::*};

#[component]
pub fn SurveyDetails(
    workspace: ReadSignal<String>,
    task: ReadSignal<String>,
) -> Element {
    let navigator = use_navigator();
    let mut details = use_signal(SurveyActivityDetails::default);

    use_hook(move || {
        api_fetch!(
            GET,
            format!("/api/v1/activities/details/{workspace}/{task}"),
            on_success = move |body: SurveyActivityDetails| details.set(body),
            on_error = move |e: shared::common::Error| ErrorService::show(t!(e.to_string()))
        )
    });
    
    rsx! {
        div {
            class: format!("flex-scrollable justify-center items-center gap-2 p-4 {class}", class = if details.read().workspace.is_empty() { "hidden" } else { "" }),
            div {
                class: "card w-full h-auto max-w-md card-border shadow-lg bg-base-100",
                div {
                    class: "card-body",
                    div {
                        class: "card-title flex text-primary text-xl gap-4 capitalize",
                        i { class: "bi bi-incognito"}
                        { t!("survey") }
                    }
                    div {
                        class: "h-0.25 bg-base-300",
                    }
                    ul {
                        class: "list w-full",
                        li {
                            class: "list-row py-1",
                            div { i { class: "bi bi-anthropic text-base-content/70" } }
                            div { "{details.read().survey_name}" }
                        }
                    }
                    div {
                        class: "card-actions justify-end mt-6",
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                SURVEY.signal().set(SurveyRecord::default());
                                navigator.push(Route::SurveyStart { workspace: workspace(), task: task()});
                            },
                            { t!("begin") }
                        }
                    }
                }
            }
            div {
                class: "flex",
                button {
                    class: "btn btn-link text-base-content/60 lowercase",
                    onclick: move |_| close_window(),
                    { t!("close") }
                }
            }
        }
    }
}