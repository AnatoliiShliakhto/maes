use crate::{
    components::{dialogs::*, inputs::*, widgets::*},
    prelude::*,
    services::*,
    window::*,
};
use ::chrono::{TimeDelta, TimeZone, Utc};
use ::std::{collections::HashSet, ops::Add};

#[derive(Copy, Clone, PartialEq)]
pub struct ReportsState {
    pub name: Signal<String>,
    pub path: Signal<String>,
    pub from: Signal<i64>,
    pub to: Signal<i64>,
    pub selected: Signal<HashSet<String>>,
    pub dialog_open: Signal<bool>,
    pub changed: Signal<i32>,
}

impl Default for ReportsState {
    fn default() -> Self {
        Self {
            name: Signal::new(String::new()),
            path: Signal::new(String::new()),
            from: Signal::new(i64::MIN),
            to: Signal::new(i64::MAX),
            selected: Signal::new(HashSet::new()),
            dialog_open: Signal::new(false),
            changed: Signal::new(0),
        }
    }
}

#[component]
pub fn Reports() -> Element {
    let mut state = use_context_provider(ReportsState::default);
    let mut reports = use_context_provider(|| Signal::new(Vec::<Entity>::new()));
    let mut dialog = use_dialog();

    use_effect(move || {
        _ = state.changed.read();
        api_fetch!(
            GET,
            "/api/v1/reports",
            on_success = move |body: Vec<Entity>| reports.set(body),
        );
    });

    let on_success_import = Callback::new(move |_| state.changed.with_mut(|c| *c += 1));
    _ = bind_task_dispatcher(Some(on_success_import), None);

    let delete_action = {
        let callback = Callback::new(move |_| {
            api_call!(
                DELETE,
                "/api/v1/reports",
                state.selected.read().iter().cloned().collect::<Vec<_>>(),
                on_success = move || {
                    reports.with_mut(|r| r.retain(|e| !state.selected.peek().contains(&e.id)));
                    state.selected.with_mut(|s| s.clear());
                },
            );
        });
        Callback::new(move |_| {
            dialog.warning(
                t!(
                    "delete-reports-message",
                    reports = state.selected.read().len()
                ),
                Some(callback),
            );
        })
    };

    let merge_action = move |_| {
        if state.selected.read().len() < 2 {
            return;
        }
        api_fetch!(
            POST,
            "/api/v1/entities/merge",
            state.selected.read().iter().cloned().collect::<Vec<_>>(),
            on_success = move |body: Entity| {
                state.selected.with_mut(|s| {
                    s.clear();
                    s.insert(body.id.clone());
                });
                reports.with_mut(|vec| vec.insert(0, body));
                ToastService::success(t!("reports-merged"))
            }
        )
    };

    rsx! {
        Panel {
            div {
                class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 items-center h-10 space-between",
                i { class: "bi bi-three-dots-vertical" }
                div { class: "w-full", { t!("reports") } }
                ul {
                    class: "menu menu-horizontal px-2 py-1 m-0 text-base-content flex-nowrap",
                    li {
                        button {
                            class: "hover:text-accent",
                            onclick: move |_| Exchange::import(),
                            i { class: "bi bi-upload" }
                            { t!("upload") }
                        }
                    }
                    li {
                        button {
                            class: format!("hover:text-primary {class}",
                                class = if state.selected.read().is_empty() { "btn-disabled bg-transparent text-base-content/50" } else { ""}
                            ),
                            onclick: move |_| {
                                if state.selected.read().is_empty() { return }
                                Exchange::export(state.selected.read().iter().cloned().collect())
                            },
                            i { class: "bi bi-floppy" }
                            { t!("download") }
                        }
                    }
                    div { class: "divider divider-horizontal m-1 w-1" }
                    li {
                        button {
                            class: format!("hover:text-warning {class}",
                                class = if state.selected.read().len() != 1 { "btn-disabled bg-transparent text-base-content/50" } else { ""}
                            ),
                            onclick: move |_| state.dialog_open.set(true),
                            i { class: "bi bi-pen" }
                            { t!("edit") }
                        }
                    }
                    li {
                        button {
                            class: format!("hover:text-accent {class}",
                                class = if state.selected.read().len() < 2 { "btn-disabled bg-transparent text-base-content/50" } else { ""}
                            ),
                            onclick: merge_action,
                            i { class: "bi bi-link-45deg" }
                            { t!("merge") }
                        }
                    }
                    div { class: "divider divider-horizontal m-1 w-1" }
                    li {
                        button {
                            class: format!("hover:text-error {class}",
                                class = if state.selected.read().is_empty() { "btn-disabled bg-transparent text-base-content/50" } else { ""}
                            ),
                            onclick: delete_action,
                            i { class: "bi bi-trash" }
                            { t!("delete") }
                        }
                    }
                }
            }
            div {
                class: "h-0.25 bg-base-300 mx-4 my-1",
            }
            div {
                class: "grid grid-cols-[1fr_max-content_1fr_1fr_1fr_max-content] gap-2 px-4 py-1",
                Calendar {
                    class: "input-sm",
                    placeholder: t!("from-date"),
                    onchange: move |evt: FormEvent| {
                        let from = parse_date_with_unknown_format(evt.value())
                        .and_then(|date| date.and_hms_opt(0, 0, 0))
                        .map(|datetime| Utc.from_utc_datetime(&datetime).timestamp())
                        .unwrap_or(i64::MIN);
                        state.from.set(from)
                    }
                }
                div { class: "flex items-center justify-center", "-" }
                Calendar {
                    class: "input-sm",
                    placeholder: t!("to-date"),
                    onchange: move |evt: FormEvent| {
                        let to = parse_date_with_unknown_format(evt.value())
                        .and_then(|date| date.and_hms_opt(0, 0, 0))
                        .map(|datetime| Utc.from_utc_datetime(&datetime).add(TimeDelta::days(1)).timestamp())
                        .unwrap_or(i64::MAX);
                        state.to.set(to)
                    }
                }
                label {
                    class: "w-full input input-sm items-center gap-2",
                    input {
                        class: "grow",
                        style: "max-width: inherit; width: 100%",
                        r#type: "search",
                        placeholder: t!("name").to_lowercase(),
                        value: "{state.name.read()}",
                        oninput: move |evt| state.name.set(evt.value()),
                    }
                    i { class: "bi bi-filter bg-base-100/0 relative -right-0" }
                }
                label {
                    class: "w-full input input-sm items-center gap-2",
                    input {
                        class: "grow",
                        style: "max-width: inherit; width: 100%",
                        r#type: "search",
                        placeholder: t!("unit").to_lowercase(),
                        value: "{state.path.read()}",
                        oninput: move |evt| state.path.set(evt.value()),
                    }
                    i { class: "bi bi-filter bg-base-100/0 relative -right-0" }
                }
                div {
                    class: "flex items-center justify-center join",
                    button {
                        class: "btn btn-sm hover:btn-error join-item",
                        onclick: move |_| {
                            state.name.set("".to_string());
                            state.path.set("".to_string());
                            clear_calendars()
                        },
                        i { class: "bi bi-eraser-fill" }
                    }
                    button {
                        class: "btn btn-sm hover:btn-success join-item",
                        onclick: move |_| {
                            let filtered = reports.read().iter().filter(|r| {
                                let is_match = state.name.read().is_empty() || r.name.to_lowercase().contains(&state.name.read().to_lowercase());
                                let is_path_match = state.path.read().is_empty() || r.path.contains(&*state.path.read());
                                let is_from = *state.from.read() == i64::MIN || r.metadata.updated_at >= *state.from.read();
                                let is_to = *state.to.read() == i64::MAX || r.metadata.updated_at <= *state.to.read();
                                is_match && is_path_match && is_from && is_to
                            }).map(|r| r.id.clone()).collect::<HashSet<_>>();
                            state.selected.with_mut(|s| s.extend(filtered))
                        },
                        i { class: "bi bi-check-square" }
                    }
                    button {
                        class: "btn btn-sm hover:btn-secondary join-item",
                        onclick: move |_| state.selected.with_mut(|s| s.clear()),
                        i { class: "bi bi-square" }
                    }
                }
            }
            RenderReportList {}
        }
        EditReportDialogContainer { key: "edit-report-container" }
    }
}

#[component]
fn RenderReportList() -> Element {
    let mut state = use_context::<ReportsState>();
    let reports = use_context::<Signal<Vec<Entity>>>();
    let reports_guard = reports.read();

    let filtered = reports_guard
        .iter()
        .filter(|r| {
            let is_match = state.name.read().is_empty()
                || r.name
                    .to_lowercase()
                    .contains(&state.name.read().to_lowercase());
            let is_path_match =
                state.path.read().is_empty() || r.path.contains(&*state.path.read());
            let is_from =
                *state.from.read() == i64::MIN || r.metadata.updated_at >= *state.from.read();
            let is_to = *state.to.read() == i64::MAX || r.metadata.updated_at <= *state.to.read();
            is_match && is_path_match && is_from && is_to
        })
        .collect::<Vec<_>>();

    let on_select = Callback::new(move |args: (EntityKind, String)| match args.0 {
        EntityKind::QuizRecord => WindowManager::open_window(
            t!("quiz-report-title"),
            WindowKind::QuizReport { entity: args.1 },
        ),
        EntityKind::SurveyRecord => WindowManager::open_window(
            t!("survey-report-title"),
            WindowKind::SurveyReport { entity: args.1 },
        ),
        _ => (),
    });

    rsx! {
        ul {
            class: "list flex-scrollable",
            for report in filtered.into_iter() {
                li {
                    key: "{report.id}",
                    class: "list-row hover:bg-base-200 rounded-none cursor-pointer",
                    onclick: {
                        let (kind, id) = (report.kind, report.id.clone());
                        move |_| on_select.call((kind, id.clone()))
                    },
                    div {
                        class: "flex flex-1 items-center justify-center",
                        onclick: move |evt| {
                            evt.stop_propagation();
                        },
                        input {
                            r#type: "checkbox",
                            class: "checkbox rounded-lg checked:checkbox-accent",
                            checked: "{state.selected.read().contains(&report.id)}",
                            onchange: {
                                let id = report.id.clone();
                                move |evt| {
                                    to_owned![id];
                                    if evt.checked() {
                                        state.selected.with_mut(|s| s.insert(id));
                                    } else {
                                        state.selected.with_mut(|s| s.remove(&id));
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "flex text-primary/40 text-xl items-center justify-center",
                        match report.kind {
                            EntityKind::QuizRecord => rsx! { i { class: "bi bi-mortarboard" } },
                            EntityKind::SurveyRecord => rsx! { i { class: "bi bi-incognito" } },
                            _ => rsx! { i { class: "bi bi-activity" } },
                        }
                    }
                    div {
                        class: "list-col-grow flex flex-col justify-center gap-1",
                        div {
                            class: "flex gap-2 justify-between items-end",
                            div {
                                class: "font-semibold",
                                "{report.name}"
                            }
                            div {
                                class: "flex flex-nowrap whitespace-nowrap inline-flex items-center text-xs opacity-60",
                                "{report.metadata.updated_at()}"
                                i { class: "bi bi-clock text-secondary ml-2" }
                            }
                        }
                        div {
                            class: "flex gap-2 justify-between items-start",
                            div {
                                class: "text-xs text-base-content/60",
                                "{report.path}"
                            }
                            div {
                                class: "flex flex-nowrap whitespace-nowrap inline-flex items-center text-xs opacity-60",
                                "{report.metadata.updated_by}"
                                i { class: "bi bi-person text-primary ml-2" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn EditReportDialogContainer() -> Element {
    let mut state = use_context::<ReportsState>();
    let mut reports = use_context::<Signal<Vec<Entity>>>();

    if !*state.dialog_open.read() {
        return rsx! {};
    };
    let Some(report_id) = state.selected.read().iter().next().cloned() else {
        return rsx! {};
    };
    let Some(report) = reports.read().iter().find(|r| r.id == report_id).cloned() else {
        return rsx! {};
    };

    let edit_action = move |evt: FormEvent| {
        evt.stop();
        let (Some(kind), Some(id), Some(name), Some(path)) =
            form_values!(evt, "kind", "id", "name", "path")
        else {
            ToastService::error(t!("missing-fields"));
            return;
        };

        let endpoint = format!("/api/v1/entities/{kind}/{id}");
        api_call!(
            PATCH,
            endpoint,
            UpdateEntityPayload {
                name: name.clone(),
                path: path.clone()
            },
            on_success = move || {
                to_owned![id, name, path];
                reports.with_mut(|vec| {
                    vec.iter_mut().find(|e| e.id == id).map(|e| {
                        e.name = name.clone();
                        e.path = path.clone();
                    })
                });
            },
        );
        state.dialog_open.set(false);
    };

    rsx! {
        dialog {
            class: "modal modal-open",
            div {
                class: "modal-box flex flex-col gap-5",
                onclick: |evt| evt.stop_propagation(),
                h3 {
                    class: "text-lg font-semibold text-accent",
                    { t!("edit-report") }
                }

                form {
                    id: "edit-report-form",
                    autocomplete: "off",
                    onsubmit: edit_action,
                    input {
                        r#type: "submit",
                        style: "position: absolute; left: -9999px; width: 1px; height: 1px;",
                        tabindex: -1,
                    }
                    input {
                        r#type: "hidden",
                        name: "kind",
                        value: "{report.kind}",
                    }
                    input {
                        r#type: "hidden",
                        name: "id",
                        value: "{report.id}",
                    }

                    fieldset {
                        class: "fieldset flex flex-col p-4 border border-base-300 rounded-(--radius-box) mt-3 gap-5",
                        legend {
                            class: "fieldset-legend",
                            i { class: "bi bi-journal-text mr-1" }
                            { t!("report") }
                        }
                        TextInputComponent {
                            label: rsx! { span { i { class: "bi bi-anthropic mr-1" } { t!("name") } } },
                            name: "name",
                            placeholder: t!("name"),
                            minlength: 5,
                            maxlength: 100,
                            required: true,
                            initial_value: "{report.name}"
                        }
                        TextArea {
                            name: "path",
                            placeholder: t!("unit"),
                            minlength: 5,
                            maxlength: 300,
                            required: true,
                            initial_value: "{report.path}"
                        }
                    }

                    div {
                        class: "flex justify-end gap-2 mt-7",
                        button {
                            class: "btn btn-ghost",
                            onclick: move |evt| {
                                evt.stop_propagation();
                                evt.prevent_default();
                                state.dialog_open.set(false);
                            },
                            { t!("no") }
                        }
                        button {
                            form: "edit-report-form",
                            class: "btn btn-primary",
                            { t!("yes") }
                        }
                    }
                }
            }
        }
    }
}
