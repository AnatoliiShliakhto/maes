use super::*;
use crate::{prelude::*, utils::*};

#[component]
pub fn SurveyRetry() -> Element {
    let navigator = use_navigator();
    let survey = SURVEY.signal();

    if survey.read().id.is_empty() {
        navigator.push(Route::Home {});
        return rsx! {};
    }

    rsx! {
        div {
            class: "flex-scrollable justify-center items-center gap-2",
            div {
                class: "card w-full h-auto max-w-md card-border shadow-lg",
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
                    p {
                        { t!("survey-finished-announcement") }
                    }
                    div {
                        class: "card-actions justify-end mt-6",
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                let survey = SURVEY.signal();
                                navigator.push(Route::SurveyStart { workspace: survey.read().workspace.clone(), task: survey.read().id.clone() });
                            },
                            { t!("retry") }
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
