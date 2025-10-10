use crate::{
    components::{inputs::*, widgets::*},
    prelude::*,
    services::*,
};

#[component]
pub fn TaskWizardStep4() -> Element {
    let mut step = use_steps().current();
    let kind = use_context::<Signal<EntityKind>>();
    let task = use_context::<Signal<(SelectedItem, SelectedItem)>>();

    let create_task_action = move |evt: FormEvent| {
        evt.stop();
        let (Some(name), Some(path), Some(ids), Some(enabled), Some(count)) =
            form_values!(evt, "name", "path", ["category_id"], ["enabled"], ["count"])
        else {
            ToastService::error(t!("missing-fields"));
            return;
        };
        let enabled = extract_form_checkboxes(&enabled);

        let categories = ids
            .into_iter()
            .zip(enabled.into_iter())
            .zip(count.into_iter())
            .filter(|((_id, enabled), _count)| *enabled)
            .map(|((id, _enabled), count)| TaskCategory {
                id,
                name: "".to_string(),
                count: count.parse::<usize>().unwrap_or(0),
                total: 0,
                checked: count.parse::<usize>().unwrap_or(0) > 0,
            })
            .collect::<Vec<_>>();

        if categories.is_empty() {
            ToastService::error(t!("no-categories-selected"));
            return
        };

        let task_guard = task.read();
        let endpoint = format!("/api/v1/tasks/{kind}", kind = kind());
        let payload = CreateTaskPayload {
            id: task_guard.0.id.clone(),
            node: task_guard.1.id.clone(),
            name,
            path,
            categories,
        };
        let on_success = move |_body: Task| {
            ToastService::success(t!("task-created"));
            use_navigator().push(Route::Tasks {});
        };
        api_fetch!(POST, endpoint, payload, on_success = on_success);
    };

    rsx! {
        div {
            class: "card flex-fixed bg-base-100 shadow",
            div {
                class: "card-body flex-fixed items-center gap-5",
                h2 {
                    class: "text-2xl font-bold",
                    { t!("task-wizard-step-4-title") }
                }
                form {
                    class: "flex-fixed w-full",
                    id: "form-create-task",
                    autocomplete: "off",
                    onsubmit: create_task_action,
                    input {
                        r#type: "submit",
                        style: "position: absolute; left: -9999px; width: 1px; height: 1px;",
                        tabindex: -1,
                    }
                    SplitPanel {
                        left_class: "shadow-none",
                        left: rsx! {
                            div {
                                class: "flex-scrollable items-center",
                                TaskWizardDetails {}
                            }
                        },
                        right_class: "shadow-none border-1 border-base-200",
                        // right_title: t!("categories"),
                        right: rsx! {
                            div {
                                class: "flex-fixed",
                                TaskWizardStep4Categories {}
                            }
                        }
                    }
                }
            }

            div {
                class: "card-actions justify-between mx-10 mb-5",
                button {
                    class: "btn",
                    onclick: move |_| step.set(step() - 1),
                    { t!("previous") }
                }
                button {
                    class: "btn btn-success",
                    form: "form-create-task",
                    { t!("run") }
                }
            }
        }
    }
}

#[component]
fn TaskWizardDetails() -> Element {
    let kind = use_context::<Signal<EntityKind>>();
    let task = use_context::<Signal<(SelectedItem, SelectedItem)>>();

    rsx! {
        div {
            class: "flex items-center text-3xl text-primary font-semibold gap-4",
            match kind() {
                EntityKind::Quiz => rsx! {
                    i { class: "bi bi-mortarboard" }
                    { t!("quiz") }
                },
                EntityKind::Survey => rsx! {
                    i { class: "bi bi-incognito" }
                    { t!("survey") }
                },
                _ => rsx! {},
            }
        }
        fieldset {
            class: "fieldset p-2 w-full mt-5",
            legend {
                class: "fieldset-legend text-sm text-primary capitalize",
                i { class: "bi bi-anthropic" }
                { t!("name") }
            }
            TextArea {
                class: "min-h-10",
                name: "name",
                required: true,
                minlength: 5,
                initial_value: "{task.read().0.name}",
            }
        }
        fieldset {
            class: "fieldset p-2 w-full",
            legend {
                class: "fieldset-legend text-sm text-primary capitalize",
                i { class: "bi bi-diagram-3" }
                { t!("unit") }
            }
            TextArea {
                class: "min-h-10",
                name: "path",
                required: true,
                minlength: 5,
                initial_value: "{task.read().1.path}",
            }
        }
    }
}

#[component]
fn TaskWizardStep4Categories() -> Element {
    let kind = use_context::<Signal<EntityKind>>();
    let task = use_context::<Signal<(SelectedItem, SelectedItem)>>();
    let mut categories = use_signal(Vec::<TaskCategory>::new);

    use_effect(move || {
        api_fetch!(
            GET,
            format!(
                "/api/v1/tasks/categories/{kind}/{id}",
                kind = kind(),
                id = task.read().0.id
            ),
            on_success = move |body: Vec<TaskCategory>| categories.set(body),
        )
    });

    rsx! {
        div {
            class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 items-center h-10 space-between",
            i { class: "bi bi-three-dots-vertical" }
            div {
                class: "w-full",
                { t!("categories") }
            }
            ul {
                class: "menu menu-horizontal p-0 m-0 text-base-content flex-nowrap",
                li {
                    button {
                        class: "hover:text-success",
                        onclick: move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            categories.with_mut(|vec| {
                                for c in vec.iter_mut() {
                                    c.checked = true;
                                }
                            })
                        },
                        i { class: "bi bi-check-square" }
                    }
                }
                li {
                    button {
                        class: "hover:text-error",
                        onclick: move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            categories.with_mut(|vec| {
                                for c in vec.iter_mut() {
                                    c.checked = false;
                                }
                            })
                        },
                        i { class: "bi bi-square" }
                    }
                }
            }
        }
        div {
            class: "h-0.25 bg-base-300 mx-4 my-1",
        }

        ul {
            class: "list w-full overflow-y-auto",
            for category in &*categories.read() {
                li {
                    key: "{category.id}",
                    class: "list-row rounded-none px-4 py-2",
                    div {
                        class: "flex items-center",
                        input {
                            r#type: "hidden",
                            name: "category_id",
                            value: "{category.id}"
                        }
                        input {
                            r#type: "checkbox",
                            class: "checkbox checked:checkbox-success rounded-sm",
                            name: "enabled",
                            value: true,
                            checked: category.checked,
                            onchange: {
                                let idx = category.id.clone();
                                move |evt| {
                                    to_owned![idx];
                                    categories.with_mut(|vec| {
                                         for c in vec.iter_mut() {
                                            if c.id == idx {
                                                c.checked = evt.checked()
                                            }
                                        }
                                    })
                                }
                            }
                        }
                        input {
                            r#type: "hidden",
                            name: "enabled",
                            value: ""
                        }
                    }
                    div {
                        class: "list-col-grow flex-1 content-center",
                        "{category.name}"
                    }
                    div {
                        class: format!("flex-nowrap gap-2 items-center {class}", class = if kind() == EntityKind::Quiz { "flex" } else { "hidden" } ),
                        div {
                            class: "tooltip tooltip-left",
                            "data-tip": t!("questions-count"),
                            // i { class: "bi bi-question-octagon text-xl text-info" }
                            TextInputComponent {
                                class: "text-lg text-base-content min-w-10",
                                r#type: "number",
                                name: "count",
                                min: 0,
                                max: category.total as i32,
                                initial_value: "{category.count}",
                            }
                        }
                    }
                }
            }
        }
    }
}
