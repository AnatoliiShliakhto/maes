use crate::{components::widgets::*, prelude::*};
use ::std::time::Duration;

#[component]
pub fn TasksList() -> Element {
    let mut selected = use_context::<Signal<SelectedItem>>();
    let mut kind = use_context::<Signal<EntityKind>>();
    let mut tasks = use_signal(Vec::<Task>::new);

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
            for task in tasks.read().iter() {
                li {
                    key: "{task.id}",
                    class: format!("list-row rounded-none px-4 py-0 cursor-pointer hover:bg-base-200 {class}", class =
                        if task.id == selected.read().id { "bg-base-300" } else { "" }),
                    onclick: {
                        let task_id = task.id.clone();
                        let task_kind = task.kind;
                        move |_| {
                            kind.set(task_kind);
                            selected.set(SelectedItem { id: task_id.clone(), ..Default::default() })
                        }
                    },
                    div {
                        class: "flex text-base-content/70 text-xl items-center justify-center",
                        match task.kind {
                            EntityKind::QuizRecord => rsx! { i { class: "bi bi-mortarboard" } },
                            EntityKind::SurveyRecord => rsx! { i { class: "bi bi-patch-question" } },
                            _ => rsx! { i { class: "bi bi-activity" } },
                        }
                    }
                    div {
                        class: "flex flex-col justify-center my-3 gap-1",
                        div {
                            class: "semibold",
                            "{task.name}"
                        }
                        div {
                            class: "text-xs text-base-content/60",
                            "{task.path}"
                        }
                    }
                    div {
                        class: "flex items-center justify-center",
                        match task.kind {
                            EntityKind::QuizRecord => rsx! {
                                ProgressCircle { key: "progress-{task.id}", progress: task.progress }
                            },
                            EntityKind::SurveyRecord => rsx! {
                                span { class: "text-lg text-base-content/60 semibold",
                                    "{task.progress}"
                                }
                            },
                            _ => rsx! {},
                        }
                    }
                }
            }
        }
    }
}
