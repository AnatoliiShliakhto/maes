use crate::{components::inputs::*, prelude::*, services::*, pages::*};
use ::std::sync::LazyLock;

static DEFAULT_QUESTION: LazyLock<QuizQuestion> = LazyLock::new(QuizQuestion::default);

#[component]
pub fn QuizEditorQuestion(category_id: ReadOnlySignal<String>, question_id: ReadOnlySignal<String>) -> Element {
    let claims = AuthService::claims();
    let mut quiz = use_context::<Signal<Quiz>>();
    let quiz_guard = quiz.read();
    let mut selected = use_context::<Signal<QuizManagerAction>>();

    let question = quiz_guard
        .categories
        .get(&*category_id.read())
        .and_then(|c| c.questions.get(&*question_id.read()))
        .unwrap_or(&DEFAULT_QUESTION);
    let mut answers = use_signal(|| question.answers.clone());

    let create_action = use_callback(move |_| {
        let id = safe_nanoid!();
        answers.write().insert(id.clone(), QuizAnswer {
            id, ..Default::default()
        });
    });

    let delete_action = use_callback(move |answer_id: String| {
        answers.write().shift_remove(&answer_id);
    });

    let save_action = move |evt: FormEvent| {
        evt.stop();
        let question_id_guard = question_id.read();
        let (Some(name), Some(answer_ids), Some(answer_names), Some(answer_correct)) =
            form_values!(evt, "name", ["answer_id"], ["answer_name"], ["answer_correct"])
        else {
            ToastService::error(t!("missing-fields"));
            return;
        };
        let correct = extract_form_checkboxes(&answer_correct);
        let answers = answer_ids
            .into_iter()
            .zip(answer_names.into_iter())
            .zip(correct.into_iter())
            .filter(|((_id, name), _)| !name.is_empty())
            .map(|((id, name), correct)| QuizAnswer {
                id,
                name,
                img: None,
                correct,
            })
            .collect::<Vec<_>>();
        if answers.len() < 2 {
            ToastService::error(t!("answers-count-error"));
            return;
        }
        if !answers.iter().any(|a| a.correct) {
            ToastService::error(t!("answer-correct-error"));
            return;
        }

        let endpoint = format!(
            "/api/v1/manager/quizzes/{quiz_id}/{category_id}/{question_id}",
            quiz_id = &quiz.read().id,
            category_id = category_id.read(),
            question_id = question_id_guard,
        );
        let payload = UpdateQuizQuestionPayload { name, answers };
        let on_success = move |body: QuizQuestion| {
            let category_id_guard = category_id.read();
            let question_id_guard = question_id.read();
            quiz.with_mut(|q| {
                let Some(category) = q.categories.get_mut(&*category_id_guard) else {
                    return;
                };
                if question_id_guard.is_empty() {
                    selected.set(QuizManagerAction::Question(category_id(), body.id.clone()));
                    category.questions.insert(body.id.clone(), body);
                    return;
                }

                let Some(question) = category.questions.get_mut(&*question_id_guard) else {
                    return;
                };
                question.name = body.name;
                question.answers = body.answers;
            });
            ToastService::success(t!("saved"));
        };

        if question_id_guard.is_empty() {
            api_fetch!(POST, endpoint, payload, on_success = on_success)
        } else {
            api_fetch!(PATCH, endpoint, payload, on_success = on_success)
        };
    };

    if answers.read().is_empty() {
        for _ in 0..2 { create_action.call(()) }
    }

    rsx! {
        div {
            class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 h-10 items-center",
            i { class: "bi bi-three-dots-vertical" }
            div {
                class: "w-full",
                { t!("question") }
            }
            if claims.is_admin() {
                ul {
                    class: "menu menu-horizontal p-0 m-0 text-base-content",
                    li {
                        button {
                            class: "hover:text-success",
                            form: "form-quiz-question-edit",
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
            class: "flex-scrollable gap-2 px-3 my-2",
            id: "form-quiz-question-edit",
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
                    { t!("quiz-question-settings") }
                }
                TextArea {
                    class: "min-h-10",
                    name: "name",
                    required: true,
                    minlength: 3,
                    placeholder: t!("question-placeholder"),
                    initial_value: "{question.name}",
                }
            }
            fieldset {
                class: "fieldset p-2",
                legend {
                    class: "fieldset-legend text-sm text-primary",
                    i { class: "bi bi-ui-checks" }
                    { t!("quiz-answers-settings") }
                    if claims.is_admin() {
                        button {
                            class: "btn btn-xs ml-2",
                            class: if answers().len() + question.answers.len() >= 10 { "disabled hidden" },
                            onclick: move |event| {
                                event.stop_propagation();
                                event.prevent_default();
                                create_action.call(());
                            },
                            i { class: "bi bi-plus-lg" }
                        }
                    }
                }
                ul {
                    class: "list w-full",
                    for (id, answer) in answers.read().iter() {
                        li {
                            key: "{id}",
                            class: "list-row rounded-none px-0 py-1 group",
                            input {
                                r#type: "hidden",
                                name: "answer_id",
                                value: "{id}"
                            }
                            div {
                                class: "flex flex-col shrink-0 items-center",
                                input {
                                    r#type: "checkbox",
                                    class: "checkbox checked:checkbox-success mt-2.5 rounded-sm",
                                    name: "answer_correct",
                                    value: true,
                                    initial_checked: answer.correct
                                }
                                input {
                                    r#type: "hidden",
                                    name: "answer_correct",
                                    value: ""
                                }
                            }
                            div {
                                TextArea {
                                    class: "min-h-10",
                                    name: "answer_name",
                                    required: true,
                                    minlength: 1,
                                    placeholder: t!("answer-placeholder"),
                                    initial_value: "{answer.name}",
                                }
                            }
                            if claims.is_admin() {
                                div {
                                    class: "hidden h-full w-14 items-center justify-center rounded-(--radius-field)",
                                    class: "bg-error/50 group-hover:flex hover:bg-error cursor-pointer",
                                    onclick: {
                                        let answer_id = id.clone();
                                        move |_| delete_action.call(answer_id.clone())
                                    },
                                    i { class: "bi bi-trash text-lg text-error-content" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
