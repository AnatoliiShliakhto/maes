use crate::{components::inputs::*, prelude::*, services::*, pages::*};
use ::std::sync::LazyLock;

static DEFAULT_CATEGORY: LazyLock<QuizCategory> = LazyLock::new(QuizCategory::default);

#[component]
pub fn QuizEditorCategory(category_id: ReadSignal<String>) -> Element {
    let claims = AuthService::claims();
    let mut quiz = use_context::<Signal<Quiz>>();
    let quiz_guard = quiz.read();
    let mut selected = use_context::<Signal<QuizManagerAction>>();

    let category = quiz_guard
        .categories
        .get(&*category_id.read())
        .unwrap_or(&DEFAULT_CATEGORY);

    let save_action = move |evt: FormEvent| {
        evt.stop();
        let (Some(name), important, Some(count), Some(order)) =
            form_values!(evt, "name", "important", "count", "order")
        else {
            ToastService::error(t!("missing-fields"));
            return;
        };

        let category_id_guard = category_id.read();
        let endpoint = format!(
            "/api/v1/manager/quizzes/{quiz_id}/{category_id_guard}",
            quiz_id = quiz.read().id
        );
        let payload = UpdateQuizCategoryPayload {
            name,
            important: important.is_some(),
            count: count.parse::<usize>().unwrap_or(0),
            order: order.parse::<usize>().unwrap_or(0),
        };
        let on_success = move |body: QuizCategory| {
            quiz.with_mut(|q| {
                let category_id_guard = category_id.read();
                if category_id_guard.is_empty() {
                    selected.set(QuizManagerAction::Category(body.id.clone()));
                    q.categories.insert(body.id.clone(), body);
                    return;
                }

                let Some(category) = q.categories.get_mut(&*category_id_guard) else {
                    return;
                };
                category.name = body.name;
                category.count = body.count;
                category.important = body.important;
                category.order = body.order;
            });
            ToastService::success(t!("saved"));
        };
        if category_id_guard.is_empty() {
            api_fetch!(POST, endpoint, payload, on_success = on_success)
        } else {
            api_fetch!(PATCH, endpoint, payload, on_success = on_success)
        };
    };

    rsx! {
        div {
            class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 h-10 items-center",
            i { class: "bi bi-three-dots-vertical" }
            div {
                class: "w-full",
                { t!("category") }
            }
            if claims.is_admin() {
                ul {
                    class: "menu menu-horizontal p-0 m-0 text-base-content",
                    li {
                        button {
                            class: "hover:text-success",
                            form: "form-quiz-category-edit",
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
            id: "form-quiz-category-edit",
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
                    { t!("quiz-category-settings") }
                }
                TextArea {
                    class: "min-h-10",
                    name: "name",
                    required: true,
                    minlength: 3,
                    placeholder: t!("category-placeholder"),
                    initial_value: "{category.name}",
                }
                div {
                    class: "grid grid-cols-[max-content_1fr] mt-2 gap-4 text-sm items-center",
                    div {
                        TextInputComponent {
                            class: "text-lg text-base-content min-w-10",
                            r#type: "number",
                            name: "count",
                            min: 0,
                            max: category.questions.len() as i32,
                            initial_value: "{category.count}",
                        }
                    }
                    div { { t!("ticket-question-count") } }
                    div {
                        TextInputComponent {
                            class: "text-lg text-base-content min-w-10",
                            r#type: "number",
                            name: "order",
                            min: 0,
                            max: 255,
                            initial_value: "{category.order}",
                        }
                    }
                    div { { t!("sort-order") } }
                    div {
                        class: "flex justify-end",
                        input {
                            r#type: "checkbox",
                            name: "important",
                            class: "toggle checked:toggle-accent",
                            initial_checked: "{category.important}",
                        }
                    }
                    div { { t!("important-category") } }
                }
            }
        }
    }
}
