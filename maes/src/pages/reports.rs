use crate::{components::{dialogs::*, widgets::*}, prelude::*, services::*, window::*};
use ::chrono::{TimeDelta, TimeZone, Utc};
use ::std::{collections::HashSet, ops::Add};

#[derive(Copy, Clone, PartialEq)]
pub struct ReportsState {
    pub name: Signal<String>,
    pub path: Signal<String>,
    pub from: Signal<i64>,
    pub to: Signal<i64>,
    pub selected: Signal<HashSet<String>>,
}

impl Default for ReportsState {
    fn default() -> Self {
        Self {
            name: Signal::new(String::new()),
            path: Signal::new(String::new()),
            from: Signal::new(i64::MIN),
            to: Signal::new(i64::MAX),
            selected: Signal::new(HashSet::new()),
        }
    }
}

#[component]
pub fn Reports() -> Element {
    let mut state = use_context_provider(ReportsState::default);
    let mut reports = use_context_provider(|| Signal::new(Vec::<Entity>::new()));
    let mut refresh_counter = use_signal(|| 0);

    use_effect(move || {
        let _ = refresh_counter.read();
        api_fetch!(
            GET,
            "/api/v1/reports",
            on_success = move |body: Vec<Entity>| reports.set(body),
        );
    });

    let on_success_import = use_callback(move |_| refresh_counter.with_mut(|c| *c += 1));
    let _ = bind_task_dispatcher(Some(on_success_import), None);

    let delete_action = {
        let callback = use_callback(move |_| {
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
        use_callback(move |_| {
            use_dialog().warning(t!("delete-reports-message"), Some(callback));
        })
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

    rsx! {
        ul {
            class: "list flex-scrollable",
            for report in filtered.into_iter() {
                li {
                    key: "{report.id}",
                    class: "list-row hover:bg-base-200 rounded-none cursor-pointer",
                    onclick: {
                        let (kind, id) = (report.kind, report.id.clone());
                        move |_| {
                            to_owned![id];
                            match kind {
                                EntityKind::QuizRecord => WindowManager::open_window(
                                    t!("quiz-report-title"),
                                    WindowKind::QuizReport { entity: id }
                                ),
                                EntityKind::SurveyRecord => WindowManager::open_window(
                                    t!("survey-report-title"),
                                    WindowKind::SurveyReport { entity: id }
                                ),
                                _ => ()
                            }
                        }
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
                        class: "flex text-base-content/70 text-xl items-center justify-center",
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
