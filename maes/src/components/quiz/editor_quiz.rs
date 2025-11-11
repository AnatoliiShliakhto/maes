use crate::{components::inputs::*, prelude::*, services::*};

#[component]
pub fn QuizEditorQuiz() -> Element {
    let claims = AuthService::claims();
    let mut quiz = use_context::<Signal<Quiz>>();
    let quiz_guard = quiz.read();

    let mut attempts = use_signal(|| quiz_guard.attempts);
    let mut duration = use_signal(|| quiz_guard.duration);
    let mut grade_a = use_signal(|| quiz_guard.grade.a);
    let mut grade_b = use_signal(|| quiz_guard.grade.b);
    let mut grade_c = use_signal(|| quiz_guard.grade.c);
    let mut grade_similarity = use_signal(|| quiz_guard.grade.similarity);

    let save_action = move |evt: FormEvent| {
        evt.stop();
        let quiz_guard = quiz.read();
        let (
            Some(name),
            Some(attempts),
            Some(duration),
            Some(grade_a),
            Some(grade_b),
            Some(grade_c),
            Some(grade_similarity),
        ) = form_values!(
            evt,
            "name",
            "attempts",
            "duration",
            "grade_a",
            "grade_b",
            "grade_c",
            "grade_similarity"
        )
        else {
            ToastService::error(t!("missing-fields"));
            return;
        };
        api_fetch!(
            PATCH,
            format!("/api/v1/manager/quizzes/{quiz_id}", quiz_id = quiz_guard.id),
            UpdateQuizPayload {
                name,
                node: quiz_guard.node.clone(),
                attempts: attempts.parse::<usize>().unwrap_or(0),
                duration: duration.parse::<i64>().unwrap_or(0),
                grade: QuizGrade {
                    a: grade_a.parse::<usize>().unwrap_or(75),
                    b: grade_b.parse::<usize>().unwrap_or(50),
                    c: grade_c.parse::<usize>().unwrap_or(25),
                    similarity: grade_similarity.parse::<usize>().unwrap_or(75),
                },
                categories: vec![],
            },
            on_success = move |body: Quiz| {
                quiz.with_mut(|q| {
                    q.name = body.name;
                    q.node = body.node;
                    q.attempts = body.attempts;
                    q.duration = body.duration;
                    q.grade = body.grade;
                });
                ToastService::success(t!("saved"))
            },
        )
    };

    let validate_images_action = move |evt: MouseEvent| {
        evt.prevent_default();
        api_fetch!(
            GET,
            format!(
                "/api/v1/manager/images/validate/{kind}/{entity_id}",
                kind = EntityKind::Quiz,
                entity_id = quiz.read().id
            ),
            on_success = move |body: Quiz| {
                quiz.set(body);
                ToastService::success(t!("images-validated"))
            }
        )
    };

    rsx! {
        div {
            class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 items-center h-10",
            i { class: "bi bi-three-dots-vertical" }
            div {
                class: "w-full",
                { t!("quiz") }
            }
            if claims.is_admin() {
                ul {
                    class: "menu menu-horizontal p-0 m-0 text-base-content flex-nowrap",
                    li {
                        button {
                            class: "hover:text-warning",
                            onclick: validate_images_action,
                            i { class: "bi bi-image" }
                            { t!("validate-images") }
                        }
                    }
                    li {
                        button {
                            class: "hover:text-success",
                            form: "form-quiz-edit",
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
            id: "form-quiz-edit",
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
                //                class: "fieldset p-4 border border-base-300 text-sm rounded-(--radius-box)",
                class: "fieldset p-2",
                legend {
                    class: "fieldset-legend text-sm text-primary",
                    i { class: "bi bi-wrench-adjustable-circle" }
                    { t!("quiz-settings") }
                }
                TextArea {
                    class: "min-h-10",
                    name: "name",
                    required: true,
                    minlength: 3,
                    maxlength: 100,
                    placeholder: t!("quiz-placeholder"),
                    initial_value: "{quiz_guard.name}",
                }

                fieldset {
                    class: "fieldset w-full mt-3",
                    legend {
                        class: "text-sm",
                        { t!("quiz-attempts", count = attempts()) }
                    }
                    div {
                        class: "w-full",
                        input {
                            class: "range range-xs w-full",
                            name: "attempts",
                            r#type: "range",
                            min: 0,
                            max: 10,
                            step: 1,
                            initial_value: "{attempts}",
                            onchange: move |evt| attempts.set(evt.value().parse::<usize>().unwrap_or_default())
                        }
                        div {
                            class: "flex justify-between mt-2 text-xs",
                            span { class: "font-semibold ml-1", "∞" } span { "1" } span { "2" } span { "3" } span { "4" } span { "5" } span { "6" } span { "7" } span { "8" } span { "9" } span { "10" }
                        }
                    }
                }

                fieldset {
                    class: "fieldset w-full mt-2",
                    legend {
                        class: "text-sm",
                        { t!("quiz-duration", total = duration(), m = duration() / 60, s = duration() % 60) }
                    }
                    div {
                        class: "w-full",
                        input {
                            class: "range range-xs w-full",
                            name: "duration",
                            r#type: "range",
                            min: 0,
                            max: 300,
                            step: 10,
                            initial_value: "{duration}",
                            onchange: move |evt| duration.set(evt.value().parse::<i64>().unwrap_or_default())
                        }
                    }
                }
            }

            fieldset {
                //                class: "fieldset p-4 border border-base-300 text-sm rounded-(--radius-box)",
                class: "fieldset p-2 text-sm",
                legend {
                    class: "fieldset-legend text-sm text-primary",
                    i { class: "bi bi-award" }
                    { t!("grade-settings") }
                }
                fieldset {
                    class: "fieldset w-full m-0",
                    legend {
                        class: "text-sm",
                        { t!("grade-a-settings", value = grade_a()) }
                    }
                    div {
                        class: "w-full",
                        input {
                            class: "range range-success range-xs w-full",
                            name: "grade_a",
                            r#type: "range",
                            min: 0,
                            max: 100,
                            step: 1,
                            initial_value: "{grade_a}",
                            onchange: move |event| grade_a.set(event.value().parse::<usize>().unwrap_or_default())
                        }
                    }
                }
                fieldset {
                    class: "fieldset w-full mt-2",
                    legend {
                        class: "text-sm",
                        { t!("grade-b-settings", value = grade_b()) }
                    }
                    div {
                        class: "w-full",
                        input {
                            class: "range range-warning range-xs w-full",
                            name: "grade_b",
                            r#type: "range",
                            min: 0,
                            max: 100,
                            step: 1,
                            initial_value: "{grade_b}",
                            onchange: move |event| grade_b.set(event.value().parse::<usize>().unwrap_or_default())
                        }
                    }
                }
                fieldset {
                    class: "fieldset w-full mt-2",
                    legend {
                        class: "text-sm",
                        { t!("grade-c-settings", value = grade_c()) }
                    }
                    div {
                        class: "w-full",
                        input {
                            class: "range range-error range-xs w-full",
                            name: "grade_c",
                            r#type: "range",
                            min: 0,
                            max: 100,
                            step: 1,
                            initial_value: "{grade_c}",
                            onchange: move |event| grade_c.set(event.value().parse::<usize>().unwrap_or_default())
                        }
                    }
                }
            }
            fieldset {
                //                class: "fieldset p-4 border border-base-300 text-sm rounded-(--radius-box)",
                class: "fieldset p-2 text-sm",
                legend {
                    class: "fieldset-legend text-sm text-primary",
                    i { class: "bi bi-openai" }
                    { t!("text-similarity-settings") }
                }
                fieldset {
                    class: "fieldset w-full m-0",
                    legend {
                        class: "text-sm",
                        { t!("grade-similarity-settings", value = grade_similarity()) }
                    }
                    div {
                        class: "w-full",
                        input {
                            class: "range range-secondary range-xs w-full",
                            name: "grade_similarity",
                            r#type: "range",
                            min: 0,
                            max: 100,
                            step: 1,
                            initial_value: "{grade_similarity}",
                            onchange: move |event| grade_similarity.set(event.value().parse::<usize>().unwrap_or_default())
                        }
                    }
                }
            }
        }
    }
}
