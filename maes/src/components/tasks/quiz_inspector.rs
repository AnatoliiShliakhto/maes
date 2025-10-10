use super::cards::*;
use crate::{prelude::*, components::widgets::*};
use ::shared::models::*;
use ::std::time::Duration;

#[component]
pub fn QuizInspector() -> Element {
    use_context_provider(|| Signal::new(QuizRecordStudent::default()));
    let kind = use_context::<Signal<EntityKind>>();
    let selected = use_context::<Signal<SelectedItem>>();
    let mut quiz = use_context_provider(|| Signal::new(QuizRecord::default()));
    let mut search_pattern = use_signal(String::new);
    let mut pinned = use_signal(|| false);

    use_future(move || async move {
        loop {
            let id = selected.read().id.clone();
            if id.is_empty() {
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
            let endpoint = format!("/api/v1/tasks/{kind}/{id}", kind = kind.read());
            api_fetch!(
                GET,
                endpoint,
                on_success = move |body: QuizRecord| quiz.set(body)
            );
            tokio::time::sleep(Duration::from_secs(5)).await
        }
    });

    rsx! {
        div {
            class: "flex w-full h-50 shrink-0 group [perspective:1000px] px-1 cursor-pointer",
            onclick: move |_| pinned.set(!pinned()),
            div {
                class: format!(
                    "relative h-full w-full transition-all duration-500 [transform-style:preserve-3d] {}",
                    if pinned() { "[transform:rotateY(180deg)]" } else { "group-hover:[transform:rotateY(180deg)]" }
                ),
                div {
                    class: "absolute inset-0",
                    i { class: "bi bi-arrow-repeat text-base-content/50 absolute top-2 right-3" }
                    div {
                        class: "card h-full w-full",
                        div { class: "card-body p-3", RenderTicketCard {} }
                    }
                }
                div {
                    class: "absolute inset-0 h-full w-full rounded-xl bg-base-100 text-base-content [transform:rotateY(180deg)] [backface-visibility:hidden]",
                    if pinned() {
                        i { class: "bi bi-pin-angle text-accent absolute top-2 left-3" }
                    } else {
                        i { class: "bi bi-arrow-repeat text-base-content/50 absolute top-2 left-3" }
                    }
                    div {
                        class: "card h-full w-full",
                        div { class: "card-body p-3", RenderWifiCard {} }
                    }
                }
            }
        }
        div {
            class: "flex shrink-0 px-3 py-4",
            label {
                class: "w-full input input-sm items-center gap-2",
                input {
                    class: "grow",
                    style: "max-width: inherit; width: 100%",
                    r#type: "search",
                    name: "pattern",
                    placeholder: t!("search"),
                    value: "{search_pattern}",
                    oninput: move |evt| search_pattern.set(evt.value()),
                }
                i { class: "bi bi-search bg-base-100/0 relative -right-0" }
            }
        }
        div {
            class: "flex-scrollable",
            ul {
                class: "list w-full",
                {
                    let pat = search_pattern.read().to_lowercase();
                    let quiz_guard = quiz.read();
                    rsx! {
                        for s in quiz_guard
                            .students
                            .values()
                            .filter(|s| s.name.to_lowercase().contains(&pat))
                        {
                            RenderStudentItem { key: "{s.id}", student: s.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderStudentItem(student: ReadSignal<QuizRecordStudent>) -> Element {
    let quiz = use_context::<Signal<QuizRecord>>();
    let student_guard = student.read();
    let mut active = use_context::<Signal<QuizRecordStudent>>();
    let is_active = student_guard.id == active.read().id;

    rsx! {
        li {
            class: format!(
                "list-row rounded-none p-0 cursor-pointer hover:bg-base-200 {class} group",
                class = if is_active { "bg-base-300" } else { "" }
            ),
            onclick: move |_| active.set(student()),
            div {
                class: "list-col-grow flex flex-col justify-center pl-4 my-3 gap-1",
                div { class: "font-semibold", "{student_guard.name}" }
                if let Some(rank) = student_guard.rank.clone() {
                    div { class: "text-xs text-base-content/60", "{rank}" }
                }
            }
            div {
                class: "flex group-hover:hidden items-center justify-center pr-4",
                Rating { grade: student_guard.grade }
            }
            div {
                class: "hidden group-hover:flex items-center justify-center w-12 cursor-pointer text-xl",
                class: "hover:bg-primary hover:text-primary-content",
                onclick: move |_| {
                    let quiz_guard = quiz.read();
                    crate::windows::mock_window(
                        format!(
                            "{host}/{kind}/{workspace_id}/{quiz_id}/{student_id}",
                            host = localhost(),
                            kind = EntityKind::QuizRecord,
                            workspace_id = quiz_guard.workspace,
                            quiz_id = quiz_guard.id,
                            student_id = student.read().id
                        )
                    )
                },
                i { class: "bi bi-phone" }
            }
        }
    }
}
