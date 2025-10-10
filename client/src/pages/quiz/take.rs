use super::*;
use crate::{prelude::*, services::*, components::*};

#[component]
pub fn QuizTake() -> Element {
    let navigator = use_navigator();
    let quiz = QUIZ.signal();
    let current = CURRENT.signal();

    if quiz.read().task.is_empty() {
        navigator.go_back();
        return rsx! {};
    }

    let quiz_guard = quiz.read();

    let Some((_id, question)) = quiz_guard.questions.get_index(current()) else {
        ErrorService::show(t!("no-question"));
        return rsx! {};
    };

    rsx! {
        div {
            class: "flex-fixed w-full",
            div {
                class: "flex shrink-0 w-full p-4 items-center gap-4",
                div {
                    class: "text-base-content/60",
                    { format!("{}/{}", current() + 1, quiz_guard.questions.len()) }
                }
                progress {
                    class: "flex flex-1 progress text-primary",
                    value: current() + 1,
                    max: quiz_guard.questions.len(),
                }
                if quiz_guard.duration > 0 {
                    Timer {
                        duration: TIMER.signal(),
                        on_expired: move |_| { navigator.push(Route::QuizFinish {}); },
                    }
                }
            }
            div {
                key: "{question.id}",
                class: "flex-scrollable bg-base-100 w-full h-full pb-16",
                ul {
                    class: "list w-full",
                    li {
                        class: "list-row flex w-full bg-base-200 rounded-none",
                        div {
                            class: "flex flex-col w-full",
                            if question.img {
                                div {
                                    class: "flex w-full max-w-md items-center justify-center mb-4",
                                    img { class: "w-full h-auto object-contain", src: format!("/images/{}/{}/{}.webp", quiz_guard.workspace, quiz_guard.quiz, question.id) }
                                }
                            }
                            div {
                                class: "flex flex-wrap font-medium text-pretty items-center",
                                "{question.name}"
                            }
                        }
                    }
                    if question.kind == QuizActivityQuestionKind::Single {
                        RenderSingleKindQuestion {
                            key: "{question.id}",
                            question: question.clone(),
                        }
                    } else {
                        RenderMultipleKindQuestion{
                            key: "{question.id}",
                            question: question.clone(),
                        }
                    }
                }
                RenderControls {}
            }
        }
    }
}

#[component]
fn RenderSingleKindQuestion(question: ReadSignal<QuizActivityQuestion>) -> Element {
    let quiz = QUIZ.signal();
    let quiz_guard = quiz.read();

    rsx! {
        for answer in question.read().answers.values() {
            li {
                class: "list-row flex w-full rounded-none transition-colors p-0",
                class: "has-[input:checked]:bg-success/20 has-[input:checked]:ring-1 has-[input:checked]:ring-success/50",
                label {
                    class: "flex w-full cursor-pointer justify-start gap-2 p-4",
                    div {
                        class: "flex h-full items-center",
                        input {
                            key: "{answer.id}",
                            r#type: "radio",
                            class: "radio radio-lg checked:radio-success",
                            checked: question.read().answered.contains(&answer.id),
                            onchange: {
                                let answer_id = answer.id.clone();
                                move |_| {
                                    QUIZ.with_mut(|quiz| {
                                        let Some(question) = quiz.questions.get_mut(&question.read().id) else { return };
                                        question.answered.clear();
                                        question.answered.insert(answer_id.clone());
                                    });
                                }
                            }
                        }
                    }
                    div {
                        class: "flex flex-col w-full justify-center",
                        if answer.img {
                            div {
                                class: "flex w-full max-w-md items-center justify-center mb-2",
                                img { class: "w-full h-auto object-contain", src: format!("/images/{}/{}/{}.webp", quiz_guard.workspace, quiz_guard.quiz, answer.id) }
                            }
                        }
                        div {
                            class: "flex w-full text-pretty",
                            "{answer.name}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderMultipleKindQuestion(question: ReadSignal<QuizActivityQuestion>) -> Element {
    let quiz = QUIZ.signal();
    let quiz_guard = quiz.read();

    rsx! {
        for answer in question.read().answers.values() {
            li {
                class: "list-row flex w-full rounded-none transition-colors p-0",
                class: "has-[input:checked]:bg-info/20 has-[input:checked]:ring-1 has-[input:checked]:ring-info/50",
                label {
                    class: "flex w-full cursor-pointer justify-start gap-2 p-4",
                    div {
                        class: "flex h-full items-center",
                        input {
                            key: "{answer.id}",
                            r#type: "checkbox",
                            class: "checkbox checkbox-lg rounded-lg checked:checkbox-info",
                            checked: question.read().answered.contains(&answer.id),
                            onchange: {
                                let answer_id = answer.id.clone();
                                move |evt: FormEvent| {
                                    QUIZ.with_mut(|quiz| {
                                        let Some(question) = quiz.questions.get_mut(&question.read().id) else { return };
                                        if evt.checked() {
                                            question.answered.insert(answer_id.clone());
                                        } else {
                                            question.answered.remove(&answer_id);
                                        }
                                    });
                                }
                            }
                        }
                    }
                    div {
                        class: "flex flex-col w-full justify-center",
                        if answer.img {
                            div {
                                class: "flex w-full max-w-md items-center justify-center mb-2",
                                img { class: "w-full h-auto object-contain", src: format!("/images/{}/{}/{}.webp", quiz_guard.workspace, quiz_guard.quiz, answer.id) }
                            }
                        }
                        div {
                            class: "flex w-full text-pretty",
                            "{answer.name}"
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
    let quiz = QUIZ.signal();
    let mut current = CURRENT.signal();
    let questions_count = quiz.read().questions.len();
    let has_answered = quiz
        .read()
        .questions
        .get_index(current())
        .map(|(_, question)| !question.answered.is_empty())
        .unwrap_or(false);

    rsx! {
        div {
            class: "flex shrink-0 w-full items-center justify-between px-8 pt-10",
            button {
                class: format!("btn btn-lg btn-primary {class}" , class = if current() == 0 { "btn-disabled" } else { "" }),
                onclick: move |_| if current() > 0 { current.set(current() - 1) },
                { t!("previous") }
            }
            if current() + 1 < questions_count {
                button {
                    class: format!("btn btn-lg btn-primary {class}", class = if !has_answered { "btn-disabled" } else { "" }),
                    onclick: move |_| if has_answered { current.set(current() + 1) },
                    { t!("next") }
                }
            } else {
                button {
                    class: format!("btn btn-lg btn-success {class}", class = if !has_answered { "btn-disabled" } else { "" }),
                    onclick: move |_| { navigator.push(Route::QuizFinish {}); },
                    { t!("finish") }
                }
            }
        }

    }
}
