use crate::{components::inputs::*, pages::*, prelude::*, services::*};
use ::indexmap::IndexMap;
use ::std::sync::LazyLock;

static DEFAULT_CATEGORY: LazyLock<SurveyCategory> = LazyLock::new(SurveyCategory::default);

#[component]
pub fn SurveyEditorCategory(category_id: ReadOnlySignal<String>) -> Element {
    let claims = AuthService::claims();
    let mut survey = use_context::<Signal<Survey>>();
    let mut selected = use_context::<Signal<SurveyManagerAction>>();
    let survey_guard = survey.read();

    let category = survey_guard
        .categories
        .get(&category_id.read().clone())
        .unwrap_or(&DEFAULT_CATEGORY);

    let mut answers = use_signal(|| category.answers.clone());
    let mut questions = use_signal(|| category.questions.clone());

    let create_answer_action = use_callback(move |_| {
        let count = if answers.read().is_empty() { 2 } else { 1 };
        answers.with_mut(|items| {
            for _ in 0..count {
                let id = safe_nanoid!();
                items.insert(id.clone(), SurveyCategoryItem { id, ..Default::default() });
            }
        });
    });

    let create_question_action = use_callback(move |_| {
        let id = safe_nanoid!();
        questions.write().insert(
            id.clone(),
            SurveyCategoryItem {
                id,
                ..Default::default()
            },
        );
    });

    let save_action = move |evt: FormEvent| {
        evt.stop();
        let (
            Some(name),
            Some(order),
            answer_ids,
            answer_names,
            Some(question_ids),
            Some(question_names),
        ) = form_values!(
            evt,
            "name",
            "order",
            ["answer_id"],
            ["answer_name"],
            ["question_id"],
            ["question_name"]
        )
        else {
            ToastService::error(t!("missing-fields"));
            return;
        };

        let answers = match (answer_ids, answer_names) {
            (Some(ids), Some(names)) => ids
                .into_iter()
                .zip(names)
                .filter(|(_id, name)| !name.is_empty())
                .map(|(id, name)| SurveyCategoryItem { id, name })
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        };

        let questions = question_ids
            .into_iter()
            .zip(question_names)
            .filter(|(_id, name)| !name.is_empty())
            .map(|(id, name)| SurveyCategoryItem { id, name })
            .collect::<Vec<_>>();

        if questions.is_empty() {
            ToastService::error(t!("questions-count-error"));
            return;
        }

        let category_id_guard = category_id.read();
        let survey_id = survey.read().id.clone();
        let endpoint = format!("/api/v1/manager/surveys/{}/{}", survey_id, category_id_guard);

        let payload = UpdateSurveyCategoryPayload {
            name,
            order: order.parse::<usize>().unwrap_or(0),
            answers,
            questions,
        };

        let on_success = move |body: SurveyCategory| {
            survey.with_mut(|s| {
                let category_id_guard = category_id.read();
                if category_id_guard.is_empty() {
                    selected.set(SurveyManagerAction::Category(body.id.clone()));
                    s.categories.insert(body.id.clone(), body);
                    return;
                }
                if let Some(category) = s.categories.get_mut(&*category_id_guard) {
                    category.name = body.name;
                    category.order = body.order;
                    category.answers = body.answers;
                    category.questions = body.questions;
                }
            });
            ToastService::success(t!("saved"));
        };

        if category_id_guard.is_empty() {
            api_fetch!(POST, endpoint, payload, on_success = on_success)
        } else {
            api_fetch!(PATCH, endpoint, payload, on_success = on_success)
        };
    };

    if questions.read().is_empty() {
        create_question_action.call(())
    }

    let is_admin = claims.is_admin();

    rsx! {
        div {
            class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 h-10 items-center",
            i { class: "bi bi-three-dots-vertical" }
            div { class: "w-full", { t!("category") } }
            if is_admin {
                ul {
                    class: "menu menu-horizontal p-0 m-0 text-base-content",
                    li {
                        button {
                            class: "hover:text-success",
                            form: "form-survey-category-edit",
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
            id: "form-survey-category-edit",
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
                class: "fieldset p-2",
                legend {
                    class: "fieldset-legend text-sm text-primary",
                    i { class: "bi bi-wrench-adjustable-circle" }
                    { t!("survey-category-settings") }
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
                            name: "order",
                            min: 0,
                            max: 255,
                            initial_value: "{category.order}",
                        }
                    }
                    div { { t!("sort-order") } }
                }
            }

            fieldset {
                class: "fieldset p-2",
                legend {
                    class: "fieldset-legend text-sm text-primary",
                    i { class: "bi bi-ui-checks" }
                    { t!("survey-answers-settings") }
                    if is_admin {
                        button {
                            class: format!("btn btn-xs ml-2 {class}", class = if answers.read().len() + category.answers.len() >= 5 { "disabled hidden" } else { "" }),
                            onclick: move |event| {
                                event.stop_propagation();
                                event.prevent_default();
                                create_answer_action.call(())
                            },
                            i { class: "bi bi-plus-lg" }
                        }
                    }
                }
                ul {
                    class: "list w-full",
                    for (id, answer) in answers.read().iter() {
                        RenderSurveyCategoryItem {
                            key: "{id}",
                            item: answer.clone(),
                            collection: answers,
                            collection_name: "answer",
                        }
                    }
                }
            }

            fieldset {
                class: "fieldset p-2",
                legend {
                    class: "fieldset-legend text-sm text-primary",
                    if answers.read().is_empty() {
                        i { class: "bi bi-list-check" }
                        { t!("survey-options-settings") }
                    } else {
                        i { class: "bi bi-question-circle" }
                        { t!("survey-questions-settings") }
                    }
                    if is_admin {
                        button {
                            class: format!("btn btn-xs ml-2 {class}", class = if questions.read().len() + category.questions.len() >= 30 { "disabled hidden" } else { "" }),
                            onclick: move |event| {
                                event.stop_propagation();
                                event.prevent_default();
                                create_question_action.call(())
                            },
                            i { class: "bi bi-plus-lg" }
                        }
                    }
                }
                ul {
                    class: "list w-full",
                    for (id, question) in questions.read().iter() {
                        RenderSurveyCategoryItem {
                            key: "{id}",
                            item: question.clone(),
                            collection: questions,
                            collection_name: "question",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderSurveyCategoryItem(
    item: ReadOnlySignal<SurveyCategoryItem>,
    mut collection: Signal<IndexMap<String, SurveyCategoryItem>>,
    collection_name: String,
) -> Element {
    let claims = AuthService::claims();
    let item_guard = item.read();

    let delete_action = use_callback(move |id: String| {
        collection.with_mut(|c| {
            if c.len() == 2 {
                c.clear()
            } else {
                c.shift_remove(&id);
            }
        })
    });

    rsx! {
        li {
            class: "list-row rounded-none px-0 py-1 group",
            div {
                class: "list-col-grow",
                input { r#type: "hidden", name: "{collection_name}_id", value: "{item_guard.id}" }
                TextArea {
                    class: "min-h-10",
                    name: "{collection_name}_name",
                    required: true,
                    minlength: 1,
                    placeholder: if collection_name == "answer" { t!("answer-placeholder") } else { t!("question-or-option-placeholder") },
                    initial_value: "{item_guard.name}",
                }
            }
            if claims.is_admin() {
                button {
                    class: "hidden group-hover:btn hover:btn-error btn-square mt-1",
                    onclick: {
                        let id = item_guard.id.clone();
                        move |evt| {
                            evt.prevent_default();
                            delete_action.call(id.clone())
                        }
                    },
                    i { class: "bi bi-trash text-lg" }
                }
            }
        }
    }
}