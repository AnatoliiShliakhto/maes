use crate::{components::{dialogs::*, widgets::*}, prelude::*};
use ::std::time::Duration;

#[component]
pub fn TasksList() -> Element {
    let mut tasks = use_context_provider(|| Signal::new(Vec::<Task>::new()));

    use_future(move || async move {
        loop {
            api_fetch!(
                GET,
                "/api/v1/tasks",
                on_success = move |body: Vec<Task>| tasks.set(body),
            );
            tokio::time::sleep(Duration::from_secs(5)).await
        }
    });

    rsx! {
        div {
            class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 items-center h-10 space-between",
            i { class: "bi bi-three-dots-vertical" }
            div {
                class: "w-full",
                { t!("tasks") }
            }
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
        div {
            class: "h-0.25 bg-base-300 mx-4 my-1",
        }

        ul {
            class: "list flex-scrollable",
            for task in tasks().into_iter() {
                RenderTaskItem { key: "{task.id}", task }
            }
        }
    }
}

#[component]
fn RenderTaskItem(task: ReadOnlySignal<Task>) -> Element {
    let mut kind = use_context::<Signal<EntityKind>>();
    let mut selected = use_context::<Signal<SelectedItem>>();
    let mut tasks = use_context::<Signal<Vec<Task>>>();
    let task_guard = task.read();

    let delete_action = {
        let callback = use_callback(move |_| {
            let task_guard = task.peek();
            api_call!(
                DELETE,
                format!("/api/v1/tasks/{kind}/{id}", kind = task_guard.kind, id = task_guard.id),
                on_success = move || {
                    tasks.with_mut(|ts| ts.retain(|t| t.id != task.peek().id));
                    if selected.read().id == task.peek().id {
                        kind.set(EntityKind::Workspace)
                    }
                },
            )
        });
        use_callback(move |_| {
            use_dialog().warning(
                t!("delete-task-message", name = task.peek().name.clone()),
                Some(callback),
            )
        })
    };

    let ctx_menu = make_ctx_menu!([
        (t!("delete"), "bi bi-trash", delete_action),
    ]);

    rsx! {
        li {
            class: format!("list-row rounded-none px-4 py-0 cursor-pointer hover:bg-base-200 {class}", class =
                if task_guard.id == selected.read().id { "bg-base-300" } else { "" }),
            onclick: move |_| {
                let task_guard = task.read();
                kind.set(task_guard.kind);
                selected.set(SelectedItem {
                    id: task_guard.id.clone(),
                    name: task_guard.name.clone(),
                    path: task_guard.path.clone(),
                })
            },
            oncontextmenu: ctx_menu,
            div {
                class: "flex text-base-content/70 text-xl items-center justify-center",
                match task_guard.kind {
                    EntityKind::QuizRecord => rsx! { i { class: "bi bi-mortarboard" } },
                    EntityKind::SurveyRecord => rsx! { i { class: "bi bi-patch-question" } },
                    _ => rsx! { i { class: "bi bi-activity" } },
                }
            }
            div {
                class: "flex flex-col justify-center my-3 gap-1",
                div {
                    class: "font-semibold",
                    "{task_guard.name}"
                }
                div {
                    class: "text-xs text-base-content/60",
                    "{task_guard.path}"
                }
            }
            div {
                class: "flex items-center justify-center",
                match task_guard.kind {
                    EntityKind::QuizRecord => rsx! {
                        ProgressCircle { key: "progress-{task_guard.id}", progress: task_guard.progress }
                    },
                    EntityKind::SurveyRecord => rsx! {
                        span { class: "text-lg text-base-content/60 font-semibold",
                            "{task_guard.progress}"
                        }
                    },
                    _ => rsx! {},
                }
            }
        }
    }
}