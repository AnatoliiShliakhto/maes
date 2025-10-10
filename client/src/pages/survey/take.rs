use super::*;
use crate::prelude::*;
use crate::services::ErrorService;

#[component]
pub fn SurveyTake() -> Element {
    let navigator = use_navigator();
    let survey = SURVEY.signal();
    let current = CURRENT.signal();

    if survey.read().id.is_empty() {
        navigator.go_back();
        return rsx! {};
    }

    let survey_guard = survey.read();
    let category_count = survey_guard.categories.len();

    let Some((_id, category)) = survey_guard.categories.get_index(current()) else {
        ErrorService::show(t!("no-category"));
        return rsx! {};
    };

    rsx! {
        div {
            class: "flex-fixed w-full",
            div {
                class: "flex shrink-0 w-full p-4 items-center gap-4",
                div {
                    class: "text-base-content/60",
                    { format!("{}/{}", current() + 1, category_count) }
                }
                progress {
                    class: "flex flex-1 progress text-primary",
                    value: current() + 1,
                    max: category_count,
                }
            }
            div {
                key: "{category.id}",
                class: "flex-scrollable bg-base-100 w-full h-full pb-16",
                div {
                    class: "flex w-full bg-base-200 rounded-none flex-wrap font-medium text-pretty items-center px-4 pb-4",
                    "{category.name}"
                }
                if category.answers.is_empty() {
                    RenderChooseCategory {
                        key: "{category.id}{current}",
                        category: category.clone(),
                    }
                } else {
                    RenderMultiplyQuestionCategory {
                        key: "{category.id}{current}",
                        category: category.clone(),
                    }
                }
                RenderControls {}
            }
        }
    }
}

#[component]
fn RenderChooseCategory(category: ReadSignal<SurveyRecordCategory>) -> Element {
    rsx! {
        ul {
            class: "list w-full",
            for (idx, (_id, question)) in category.read().questions.iter().enumerate() {
                li {
                    class: "list-row flex w-full rounded-none transition-colors p-0",
                    class: "has-[input:checked]:bg-info/20 has-[input:checked]:ring-1 has-[input:checked]:ring-info/50",
                    label {
                        class: "flex w-full cursor-pointer justify-start gap-2 p-4",
                        div {
                            class: "flex h-full items-center",
                            input {
                                key: "{_id}",
                                r#type: "checkbox",
                                class: "checkbox checkbox-lg rounded-lg checked:checkbox-info",
                                checked: *category.read().results.get(idx, 0) > 0,
                                onchange: {
                                    to_owned![idx];
                                    move |evt| {
                                        to_owned![idx];
                                        let res = if evt.checked() { 1 } else { 0 };
                                        SURVEY.with_mut(|survey| {
                                            if let Some(cat) = survey.categories.get_mut(&category.read().id) {
                                                cat.results.set(idx, 0, res as usize)
                                            }
                                        })
                                    }
                                }
                            }
                        }
                        div {
                            class: "flex w-full text-pretty items-center",
                            "{question.name}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderMultiplyQuestionCategory(category: ReadSignal<SurveyRecordCategory>) -> Element {

    rsx! {
        for (question_idx, (_question_id, question)) in category.read().questions.iter().enumerate() {
            div {
                class: "flex w-full text-pretty items-center font-medium p-4",
                "{question.name}"
            }
            ul {
                class: "list w-full",
                for (answer_idx, (_answer_id, answer)) in category.read().answers.iter().enumerate() {
                    li {
                        class: "list-row flex w-full rounded-none transition-colors p-0",
                        class: "has-[input:checked]:bg-success/20 has-[input:checked]:ring-1 has-[input:checked]:ring-success/50",
                        label {
                            class: "flex w-full cursor-pointer justify-start gap-2 p-4",
                            div {
                                class: "flex h-full items-center",
                                input {
                                    key: "{_question_id}{_answer_id}",
                                    r#type: "radio",
                                    class: "radio radio-lg checked:radio-success",
                                    checked: *category.read().results.get(question_idx, answer_idx) > 0,
                                    onchange: {
                                        to_owned![question_idx, answer_idx];
                                        move |_evt| {
                                            to_owned![question_idx, answer_idx];
                                            SURVEY.with_mut(|survey| {
                                                if let Some(cat) = survey.categories.get_mut(&category.read().id) {
                                                    cat.results.fill_row(question_idx, 0);
                                                    cat.results.set(question_idx, answer_idx, 1)
                                                }
                                            })
                                        }
                                    }
                                }
                            }
                            div {
                                class: "flex w-full text-pretty items-center",
                                "{answer.name}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderControls() -> Element {
    let navigator = use_navigator();
    let mut current = CURRENT.signal();
    let category_count = SURVEY.peek().categories.len();

    rsx! {
        div {
            class: "flex shrink-0 w-full items-center justify-between px-8 pt-10",
            button {
                class: format!("btn btn-lg btn-primary {class}" , class = if current() == 0 { "btn-disabled" } else { "" }),
                onclick: move |_| if current() > 0 { current.set(current() - 1) },
                { t!("previous") }
            }
            if current() + 1 < category_count {
                button {
                    class: "btn btn-lg btn-primary",
                    onclick: move |_| current.set(current() + 1),
                    { t!("next") }
                }
            } else {
                button {
                    class: "btn btn-lg btn-success",
                    onclick: move |_| { navigator.push(Route::SurveyFinish {}); },
                    { t!("finish") }
                }
            }
        }
    }
}
