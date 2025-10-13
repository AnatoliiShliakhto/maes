use super::*;
use crate::{prelude::*, services::*, utils::*};

#[component]
pub fn QuizDetails(
    workspace: ReadSignal<String>,
    task: ReadSignal<String>,
    student: ReadSignal<String>,
) -> Element {
    let navigator = use_navigator();
    let mut details = use_signal(QuizActivityDetails::default);

    use_hook(move || {
        api_fetch!(
            GET,
            format!("/api/v1/activities/details/{workspace}/{task}/{student}"),
            on_success = move |body: QuizActivityDetails| details.set(body),
            on_error = move |e: shared::common::Error| ErrorService::show(t!(e.to_string()))
        )
    });

    rsx! {
        div {
            class: format!("flex-scrollable justify-center items-center gap-2 {class}", class = if details.read().workspace.is_empty() { "hidden" } else { "" }),
            div {
                class: "card w-full h-auto max-w-md card-border shadow-lg",
                div {
                    class: "card-body",
                    div {
                        class: "card-title flex text-primary text-xl gap-4 capitalize",
                        i { class: "bi bi-mortarboard"}
                        { t!("quiz") }
                    }
                    div {
                        class: "h-0.25 bg-base-300",
                    }
                    ul {
                        class: "list w-full",
                        li {
                            class: "list-row py-1",
                            div { i { class: "bi bi-anthropic text-base-content/70" } }
                            div { "{details.read().quiz_name}" }
                        }
                        if details.read().duration > 0 {
                            li {
                                class: "list-row py-1",
                                div { i { class: "bi bi-stopwatch text-base-content/70" } }
                                div {{
                                    let duration = details.read().duration / 60;
                                    t!("duration", total = duration, h = duration / 60, m = duration % 60)
                                }}
                            }
                        }
                        if let Some(rank) = &details.read().student_rank {
                            li {
                                class: "list-row py-1",
                                div { i { class: "bi bi-star-fill text-base-content/70" } }
                                div { "{rank}" }
                            }
                        }
                        li {
                            class: "list-row py-1",
                            div { i { class: "bi bi-person-fill text-base-content/70" } }
                            div { class: "font-medium", "{details.read().student_name}" }
                        }
                        if details.read().grade > 0 {
                            li {
                                class: format!("list-row py-1 {class}", class = match details.read().grade {
                                    5 => "text-success",
                                    4 => "text-info",
                                    3 => "text-warning",
                                    _ => "text-error",
                                }),
                                div { i { class: "bi bi-award-fill" } }
                                div { { t!("grade", grade = details.read().grade) } }
                            }
                            li {
                                class: "list-row py-1",
                                div { i { class: "bi bi-check2-square text-base-content/70" } }
                                div { { t!("score", score = details.read().score) } }
                            }
                        }
                    }
                    div {
                        class: "card-actions justify-end mt-6",
                        if details.read().can_take {
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    QUIZ.signal().set(QuizActivity::default());
                                    CURRENT.signal().set(0);
                                    TIMER.signal().set(0);
                                    navigator.push(Route::QuizStart { workspace: workspace(), task: task(), student: student()});
                                },
                                if details.read().grade > 0 {
                                    { t!("try-again") }
                                } else {
                                    { t!("begin") }
                                }

                            }
                        } else {
                            button {
                                class: "btn btn-neutral text-base-content/60",
                                onclick: move |_| close_window(),
                                { t!("acquainted") }
                            }
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
