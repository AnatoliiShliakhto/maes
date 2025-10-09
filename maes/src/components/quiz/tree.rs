use crate::{pages::*, prelude::*, services::*, components::dialogs::*};

#[component]
pub fn QuizTree() -> Element {
    let claims = AuthService::claims();

    let mut quiz = use_context::<Signal<Quiz>>();
    let mut selected = use_context::<Signal<QuizManagerAction>>();
    let quiz_guard = quiz.read();
    let node_class = if QuizManagerAction::Quiz == *selected.read() {
        "bg-base-300"
    } else {
        ""
    };

    let create_category_action =
        use_callback(move |_| {
            ToastService::info(t!("fill-form-message"));
            selected.set(QuizManagerAction::Category("".to_string()))
        });

    let copy_categories_action = use_callback(move |_| {
        let categories = quiz.read().categories.values().cloned().collect::<Vec<_>>();
        if Clipboard::copy_json(categories).is_ok() {
            ToastService::success(t!("copy-to-clipboard-success"))
        } else {
            ToastService::error(t!("copy-to-clipboard-error"))
        }
    });

    let paste_categories_action = use_callback(move |_| {
        let quiz_guard = quiz.read();
        let Ok(categories) = Clipboard::paste_json::<Vec<QuizCategory>>() else {
            ToastService::error(t!("paste-from-clipboard-error"));
            return
        };
        api_fetch!(
            PATCH,
            format!("/api/v1/manager/quizzes/{quiz_id}", quiz_id = quiz_guard.id),
            UpdateQuizPayload {
                name: quiz_guard.name.clone(),
                node: quiz_guard.node.clone(),
                attempts: quiz_guard.attempts,
                duration: quiz_guard.duration,
                grade: quiz_guard.grade.clone(),
                categories: categories.clone(),
            },
            on_success = move |_body: Quiz| {
                quiz.with_mut(|q| {
                    for category in categories {
                        q.categories.insert(category.id.clone(), category);
                    }
                });
                ToastService::success(t!("paste-from-clipboard-success"));
            }
        );
    });

    let ctx_menu = make_ctx_menu!([
        (t!("create-quiz-category"),
        "bi bi-folder-plus",
        create_category_action,
        false,
        true),
        (t!("copy-to-clipboard"), "bi bi-clipboard-plus", copy_categories_action),
        (t!("paste-from-clipboard"), "bi bi-clipboard", paste_categories_action),
    ]);

    rsx! {
        ul {
            class: "menu flex-wrap",
            li {
                key: "{quiz_guard.id}",
                div {
                    class: "font-semibold text-primary {node_class}",
                    oncontextmenu: move |evt| {
                        if claims.is_admin() { ctx_menu(evt) } else { evt.stop_propagation() }
                    },
                    onclick: move |_| selected.set(QuizManagerAction::Quiz),
                    i { class: "bi bi-mortarboard" }
                    "{quiz_guard.name}"
                }
                ul {
                    for (category_id, _category) in quiz.read().categories.iter() {
                        RenderQuizTreeCategory {
                            key: "{category_id}",
                            category_id: "{category_id}",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderQuizTreeCategory(category_id: ReadSignal<String>) -> Element {
    let claims = AuthService::claims();
    let mut quiz = use_context::<Signal<Quiz>>();
    let mut selected = use_context::<Signal<QuizManagerAction>>();
    let quiz_guard = quiz.read();

    let Some(category) = quiz_guard.categories.get(&*category_id.read()) else {
        return rsx! {};
    };
    let node_class = match &*selected.read() {
        QuizManagerAction::Category(id) if id == &*category_id.read() => "bg-base-300",
        _ => "",
    };

    let create_question_action = use_callback(move |_| {
        ToastService::info(t!("fill-form-message"));
        selected.set(QuizManagerAction::Question(category_id(), "".to_string()))
    });

    let copy_category_action = use_callback(move |_| {
        if let Some(category) = quiz.read().categories.get(&*category_id.read())
            && Clipboard::copy_json(vec![category.clone()]).is_ok() {
            ToastService::success(t!("copy-to-clipboard-success"))
        } else {
            ToastService::error(t!("copy-to-clipboard-error"))
        }
    });

    let delete_category_action = {
        let category_name = category.name.clone();
        let callback = use_callback(move |_| {
            api_fetch!(
                DELETE,
                format!(
                    "/api/v1/manager/quizzes/{quiz_id}/{category_id}",
                    quiz_id = quiz.read().id,
                    category_id = category_id.read()
                ),
                on_success = move |body: String| {
                    quiz.with_mut(|q| {
                        q.categories.shift_remove(&body);
                    });
                    if body == *category_id.read() {
                        selected.set(QuizManagerAction::Quiz);
                    }
                }
            )
        });
        use_callback(move |_| {
            use_dialog().warning(
                t!("delete-quiz-category-message", name = category_name.clone()),
                Some(callback),
            )
        })
    };

    let ctx_menu = make_ctx_menu!([
        (
            t!("create-quiz-question"),
            "bi bi-question-square",
            create_question_action,
            false,
            true
        ),
        (t!("copy-to-clipboard"), "bi bi-clipboard-plus", copy_category_action),
        (t!("delete"), "bi bi-trash", delete_category_action),
    ]);

    let select_action =
        move |_| selected.set(QuizManagerAction::Category(category_id.read().clone()));

    rsx! {
        if category.questions.is_empty() {
            li {
                div {
                    class: "{node_class}",
                    onclick: select_action,
                    oncontextmenu: move |evt| if claims.is_admin() { ctx_menu(evt) } else { evt.stop_propagation() },
                    i { class: "bi bi-folder text-base-content/70" }
                    "{category.name}"
                }
            }
        } else {
            li {
                details {
                    summary {
                        class: "{node_class}",
                        onclick: select_action,
                        oncontextmenu: move |evt| if claims.is_admin() { ctx_menu(evt) } else { evt.stop_propagation() },
                        i { class: "bi bi-folder2-open text-base-content/70" }
                        "{category.name} [{category.questions.len()}]"
                    }
                    ul {
                        for (question_id, _question) in category.questions.iter() {
                            RenderQuizTreeQuestion {
                                key: "{question_id}",
                                category_id: "{category_id}",
                                question_id: "{question_id}",
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderQuizTreeQuestion(
    category_id: ReadSignal<String>,
    question_id: ReadSignal<String>,
) -> Element {
    let claims = AuthService::claims();
    let mut quiz = use_context::<Signal<Quiz>>();
    let mut selected = use_context::<Signal<QuizManagerAction>>();
    let quiz_guard = quiz.read();

    let Some(question) = quiz_guard
        .categories
        .get(&*category_id.read())
        .and_then(|c| c.questions.get(&*question_id.read()))
    else {
        return rsx! {};
    };
    let node_class = match &*selected.read() {
        QuizManagerAction::Question(_, id) if id == &*question_id.read() => "bg-base-300",
        _ => "",
    };

    let delete_question_action = {
        let question_name = question.name.clone();
        let callback = use_callback(move |_| {
            api_fetch!(
                DELETE,
                format!(
                    "/api/v1/manager/quizzes/{quiz_id}/{category_id}/{question_id}",
                    quiz_id = quiz.read().id,
                    category_id = category_id.read(),
                    question_id = question_id.read(),
                ),
                on_success = move |body: String| {
                    quiz.with_mut(|q| {
                        if let Some(category) = q
                        .categories
                        .get_mut(&*category_id.read()) {
                            category.questions.shift_remove(&body);
                        }
                    });
                    if body == *question_id.read() {
                        selected.set(QuizManagerAction::Category(category_id()));
                    }
                }
            )
        });
        use_callback(move |_| {
            use_dialog().warning(
                t!("delete-quiz-question-message", name = question_name.clone()),
                Some(callback),
            )
        })
    };

    let select_action =
        move |_| selected.set(QuizManagerAction::Question(category_id(), question_id()));

    let ctx_menu = make_ctx_menu!([
        (t!("delete"), "bi bi-trash", delete_question_action),
    ]);

    rsx! {
        li {
            div {
                class: "{node_class}",
                onclick: select_action,
                oncontextmenu: move |evt| if claims.is_admin() { ctx_menu(evt) } else { evt.stop_propagation() },
                i { class: "bi bi-question-square text-base-content/70" }
                "{question.name}"
            }
        }
    }
}
