use crate::{
    components::{dialogs::*, inputs::*},
    pages::*,
    prelude::*,
    services::*,
};
use ::std::sync::LazyLock;

#[component]
pub fn QuizEditorQuestion(
    category_id: ReadSignal<String>,
    question_id: ReadSignal<String>,
) -> Element {
    let claims = AuthService::claims();
    let mut quiz = use_context::<Signal<Quiz>>();
    let quiz_guard = quiz.read();
    let mut selected = use_context::<Signal<QuizManagerAction>>();

    let category_id_val = category_id.read().to_string();
    let question_id_val = question_id.read().to_string();

    let default_question = QuizQuestion {
        id: question_id(),
        ..Default::default()
    };
    let question = quiz_guard
        .categories
        .get(&category_id_val)
        .and_then(|c| c.questions.get(&question_id_val))
        .unwrap_or(&default_question);

    let mut answers = use_signal(|| question.answers.clone());
    let mut has_img = use_signal(|| question.img);

    let create_action = use_callback(move |_| {
        let id = safe_nanoid!();
        answers.write().insert(
            id.clone(),
            QuizAnswer {
                id,
                ..Default::default()
            },
        );
    });

    let delete_action = use_callback(move |answer_id: String| {
        answers.write().shift_remove(&answer_id);
    });

    let save_action = move |evt: FormEvent| {
        evt.stop();
        let question_id_guard = question_id.read();
        let (
            Some(name),
            Some(answer_ids),
            Some(answer_names),
            Some(answer_correct),
            Some(answer_img),
        ) = form_values!(
            evt,
            "name",
            ["answer_id"],
            ["answer_name"],
            ["answer_correct"],
            ["answer_img"]
        )
        else {
            ToastService::error(t!("missing-fields"));
            return;
        };

        let correct = extract_form_checkboxes(&answer_correct);
        let answers = answer_ids
            .into_iter()
            .zip(answer_names)
            .zip(correct)
            .zip(answer_img)
            .filter(|(((_id, name), _), img)| !name.is_empty() || img == "true")
            .map(|(((id, name), correct), img)| QuizAnswer {
                id,
                name,
                img: img == "true",
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

        let quiz_guard = quiz.read();
        let endpoint = format!(
            "/api/v1/manager/quizzes/{quiz_id}/{category_id}/{question_id}",
            quiz_id = quiz_guard.id,
            category_id = category_id.read(),
            question_id = question_id_guard
        );
        let payload = UpdateQuizQuestionPayload {
            name,
            img: has_img(),
            answers,
        };

        let on_success = move |body: QuizQuestion| {
            let category_id_guard = category_id.read();
            let question_id_guard = question_id.read();
            quiz.with_mut(|q| {
                let Some(category) = q.categories.get_mut(&*category_id_guard) else {
                    return;
                };
                if let Some(question) = category.questions.get_mut(&*question_id_guard) {
                    question.name = body.name;
                    question.img = body.img;
                    question.answers = body.answers;
                } else {
                    selected.set(QuizManagerAction::Question(category_id(), body.id.clone()));
                    category.questions.insert(body.id.clone(), body);
                }
            });
            ToastService::success(t!("saved"));
        };

        if question_id_guard.is_empty() {
            api_fetch!(POST, endpoint, payload, on_success = on_success)
        } else {
            api_fetch!(PATCH, endpoint, payload, on_success = on_success)
        };
    };

    let add_image_action = use_callback(move |item: (String, Callback)| {
        add_image_dialog(quiz.read().id.clone(), item.0, item.1)
    });

    let remove_image_action = use_callback(move |item: (String, Callback)| {
        api_call!(
            DELETE,
            format!("/api/v1/manager/images/{}/{}", quiz.read().id, item.0),
            on_success = move || item.1.call(())
        )
    });

    if answers.read().is_empty() {
        for _ in 0..2 {
            create_action.call(())
        }
    }

    let is_admin = claims.is_admin();
    let ws = quiz.read().workspace.clone();
    let quiz_id = quiz.read().id.clone();

    rsx! {
        div {
            class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 h-10 items-center",
            i { class: "bi bi-three-dots-vertical" }
            div { class: "w-full", { t!("question") } }
            if is_admin {
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
        div { class: "h-0.25 bg-base-300 mx-4 my-1" }

        form {
            class: "flex-scrollable gap-2 px-3 my-2",
            id: "form-quiz-question-edit",
            autocomplete: "off",
            onsubmit: move |evt| {
                if is_admin { save_action(evt) } else { evt.prevent_default() }
            },
            input {
                r#type: "submit",
                style: "position: absolute; left: -9999px; width: 1px; height: 1px;",
                tabindex: -1,
            }

            fieldset {
                class: "fieldset p-2 group",
                legend {
                    class: "fieldset-legend text-sm text-primary",
                    i { class: "bi bi-wrench-adjustable-circle" }
                    { t!("quiz-question-settings") }
                }
                if has_img() {
                    div {
                        class: "flex bg-base-200 items-center justify-center rounded-(--radius-box)",
                        div {
                            class: "max-w-50 w-full p-2",
                            img { class: "w-full h-auto object-contain", src: format!("{}/images/{}/{}/{}.webp", localhost(), ws, quiz_id, question_id.read()) }
                        }
                    }
                }
                div {
                    class: "flex w-full gap-4",
                    TextArea {
                        class: "min-h-10",
                        name: "name",
                        required: true,
                        minlength: 3,
                        placeholder: t!("question-placeholder"),
                        initial_value: "{question.name}",
                    }
                    if is_admin {
                        div {
                            class: "hidden h-full group-hover:flex",
                            button {
                                class: if has_img() { "btn hover:btn-error btn-square mt-1" } else { "btn hover:btn-info btn-square mt-1" },
                                onclick: move |evt| {
                                    evt.prevent_default();
                                    if has_img() {
                                        let on_success = use_callback(move |_| has_img.set(false));
                                        remove_image_action.call((question_id(), on_success));
                                    } else {
                                        let on_success = use_callback(move |_| has_img.set(true));
                                        add_image_action.call((question_id(), on_success));
                                    }
                                },
                                i { class: "bi bi-image text-lg" }
                            }
                        }
                    }
                }
            }

            fieldset {
                class: "fieldset p-2",
                legend {
                    class: "fieldset-legend text-sm text-primary",
                    i { class: "bi bi-ui-checks" }
                    { t!("quiz-answers-settings") }
                    if is_admin {
                        button {
                            class: format!("btn btn-xs ml-2 {class}", class = if answers.read().len() >= 10 { "disabled hidden" } else { "" }),
                            onclick: move |event| {
                                event.stop_propagation();
                                event.prevent_default();
                                create_action.call(())
                            },
                            i { class: "bi bi-plus-lg" }
                        }
                    }
                }
                ul {
                    class: "list w-full",
                    for (id, answer) in answers() {
                        li {
                            key: "{id}",
                            class: "list-row rounded-none px-0 py-1 group",
                            div {
                                class: "flex flex-col shrink-0 items-center justify-center",
                            input { r#type: "hidden", name: "answer_id", value: "{id}" }
                                input {
                                    r#type: "checkbox",
                                    class: "checkbox checked:checkbox-success rounded-sm",
                                    name: "answer_correct",
                                    value: true,
                                    initial_checked: answer.correct
                                }
                                input { r#type: "hidden", name: "answer_correct", value: "" }
                                input { r#type: "hidden", name: "answer_img", value: if answer.img { "true" } else { "false" } }
                            }
                            div {
                                class: "flex flex-col",
                                if answer.img {
                                    div {
                                        class: "flex bg-base-200 items-center justify-center rounded-(--radius-box)",
                                        div {
                                            class: "max-w-50 w-full p-2",
                                            img { class: "w-full h-auto object-contain", src: format!("{}/images/{}/{}/{}.webp", localhost(), ws, quiz_id, id) }
                                        }
                                    }
                                }
                                TextArea {
                                    class: "min-h-10",
                                    name: "answer_name",
                                    required: false,
                                    minlength: 0,
                                    placeholder: t!("answer-placeholder"),
                                    initial_value: "{answer.name}",
                                }
                            }
                            if is_admin {
                                div {
                                    class: format!("hidden group-hover:flex join {class} pt-1", class = if answer.img { "flex-col join-vertical" } else { "" }),
                                    button {
                                        class: format!("btn {class} join-item", class = if answer.img { "hover:btn-error" } else { "hover:btn-info" }),
                                        onclick: {
                                            let answer_id = answer.id.clone();
                                            let answer_id_clone = answer_id.clone();
                                            move |evt| {
                                                evt.prevent_default();
                                                to_owned![answer_id, answer_id_clone];
                                                let on_success = use_callback(move |_| {
                                                    to_owned![answer_id];
                                                    answers.with_mut(|map| {
                                                        if let Some(a) = map.get_mut(&answer_id) {
                                                            a.img = !a.img;
                                                        }
                                                    });
                                                });
                                                if answer.img {
                                                    remove_image_action.call((answer_id_clone, on_success))
                                                } else {
                                                    add_image_action.call((answer_id_clone, on_success))
                                                }
                                            }
                                        },
                                        i { class: "bi bi-image text-lg" }
                                    }
                                    button {
                                        class: "btn hover:btn-error join-item",
                                        onclick: {
                                            let answer_id = id.clone();
                                            move |evt| {
                                                evt.prevent_default();
                                                delete_action.call(answer_id.clone())
                                            }
                                        },
                                        i { class: "bi bi-trash text-lg" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
