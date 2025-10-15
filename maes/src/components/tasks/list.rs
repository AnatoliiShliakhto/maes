use crate::{
    components::{dialogs::*, widgets::*},
    prelude::*,
    window::*,
};
use ::std::time::Duration;

#[component]
pub fn TasksList() -> Element {
    let mut tasks = use_context_provider(|| Signal::new(Vec::<Task>::new()));

    use_future(move || async move {
        loop {
            api_fetch!(
                GET,
                "/api/v1/tasks",
                on_success = move |body: Vec<Task>| tasks.set(body)
            );
            tokio::time::sleep(Duration::from_secs(5)).await
        }
    });

    rsx! {
        div {
            class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 items-center h-10 space-between",
            i { class: "bi bi-three-dots-vertical" }
            div { class: "w-full", { t!("tasks") } }
            ul {
                class: "menu menu-horizontal p-0 m-0 text-base-content flex-nowrap",
                li {
                    button {
                        class: "hover:text-success",
                        onclick: move |_| { use_navigator().push(Route::TaskWizard {}); },
                        i { class: "bi bi-magic" }
                        { t!("create") }
                    }
                }
            }
        }
        div { class: "h-0.25 bg-base-300 mx-4 my-1" }

        ul {
            class: "list flex-scrollable",
            for task in tasks().into_iter() {
                RenderTaskItem { key: "{task.id}", task }
            }
        }
    }
}

#[component]
fn RenderTaskItem(task: ReadSignal<Task>) -> Element {
    let mut kind = use_context::<Signal<EntityKind>>();
    let mut selected = use_context::<Signal<SelectedItem>>();
    let mut tasks = use_context::<Signal<Vec<Task>>>();
    let task_guard = task.read();

    let is_selected = task_guard.id == selected.read().id;

    let delete_action = {
        let callback = use_callback(move |_| {
            let task_guard = task.peek();
            let endpoint = format!(
                "/api/v1/tasks/{kind}/{task_id}",
                kind = task_guard.kind,
                task_id = task_guard.id
            );
            api_call!(
                DELETE,
                endpoint,
                on_success = move || {
                    let id = task.peek().id.clone();
                    tasks.with_mut(|ts| ts.retain(|t| t.id != id));
                    if selected.read().id == id {
                        kind.set(EntityKind::Workspace)
                    }
                },
            )
        });
        use_callback(move |_| {
            let name = task.peek().name.clone();
            use_dialog().warning(t!("delete-task-message", name = name), Some(callback))
        })
    };

    let wifi_report_action = use_callback(move |_| {
        WindowManager::open_window(t!("wifi-instruction"), WindowKind::WiFiInstruction)
    });

    let ctx_menu = match task.peek().kind {
        EntityKind::QuizRecord => {
            let report_action = use_callback(move |_| {
                let task_guard = task.read();
                WindowManager::open_window(
                    t!("quiz-report-title"),
                    WindowKind::QuizReport {
                        entity: task_guard.id.clone(),
                    },
                )
            });

            let tickets_report_action = use_callback(move |_| {
                let task_guard = task.read();
                WindowManager::open_window(
                    t!("quiz-tickets-title"),
                    WindowKind::QuizTickets {
                        task: task_guard.id.clone(),
                    },
                )
            });

            make_ctx_menu!([
                (t!("report"), "bi bi-journal-text", report_action),
                (t!("quiz-tickets"), "bi bi-ticket", tickets_report_action),
                (t!("instruction"), "bi bi-wifi", wifi_report_action, false, true),
                (t!("delete"), "bi bi-trash", delete_action),
            ])
        }
        EntityKind::SurveyRecord => {
            let report_action = use_callback(move |_| {
                let task_guard = task.read();
                WindowManager::open_window(
                    t!("survey-report-title"),
                    WindowKind::SurveyReport {
                        entity: task_guard.id.clone(),
                    },
                )
            });

            let tickets_report_action = use_callback(move |_| {
                let task_guard = task.read();
                WindowManager::open_window(
                    t!("survey-tickets-title"),
                    WindowKind::SurveyTickets {
                        task: task_guard.id.clone(),
                    },
                )
            });

            make_ctx_menu!([
                (t!("report"), "bi bi-journal-text", report_action),
                (t!("survey-tickets"), "bi bi-ticket", tickets_report_action),
                (t!("instruction"), "bi bi-wifi", wifi_report_action, false, true),
                (t!("delete"), "bi bi-trash", delete_action),
            ])
        }
        _ => use_callback(move |evt: MouseEvent| {
            evt.prevent_default();
            evt.stop_propagation();
        }),
    };

    rsx! {
        li {
            class: format!(
                "list-row rounded-none px-4 py-0 cursor-pointer hover:bg-base-200 {class}",
                class = if is_selected { "bg-base-300" } else { "" }
            ),
            onclick: move |_| {
                let task_guard = task.read();
                kind.set(task_guard.kind);
                selected.set(SelectedItem {
                    id: task_guard.id.clone(),
                    name: task_guard.name.clone(),
                    path: task_guard.workspace.clone(),
                })
            },
            oncontextmenu: ctx_menu,
            div {
                class: "flex text-base-content/70 text-xl items-center justify-center",
                match task_guard.kind {
                    EntityKind::QuizRecord => rsx! { i { class: "bi bi-mortarboard" } },
                    EntityKind::SurveyRecord => rsx! { i { class: "bi bi-incognito" } },
                    _ => rsx! { i { class: "bi bi-activity" } },
                }
            }
            div {
                class: "flex flex-col justify-center my-3 gap-1",
                div { class: "font-semibold", "{task_guard.name}" }
                div { class: "text-xs text-base-content/60", "{task_guard.path}" }
            }
            div {
                class: "flex items-center justify-center",
                match task_guard.kind {
                    EntityKind::QuizRecord => rsx! {
                        ProgressCircle { key: "progress-{task_guard.id}", progress: task_guard.progress }
                    },
                    EntityKind::SurveyRecord => rsx! {
                        div { class: "badge badge-lg", "{task_guard.progress}" }
                        //span { class: "text-lg text-base-content/60 font-semibold", "{task_guard.progress}" }
                    },
                    _ => rsx! {},
                }
            }
        }
    }
}
