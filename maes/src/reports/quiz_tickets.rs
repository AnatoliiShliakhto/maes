use crate::{prelude::*, services::*};

#[component]
pub fn QuizTickets(task: ReadSignal<String>) -> Element {
    let config = ConfigService::read();
    let mut quiz = use_signal(QuizRecord::default);
    let quiz_guard = quiz.read();

    use_effect(move || {
        api_fetch!(
            GET,
            format!(
                "/api/v1/entities/payload/{kind}/{id}",
                kind = EntityKind::QuizRecord,
                id = task.read()
            ),
            on_success = move |body: QuizRecord| quiz.set(body)
        );
    });

    let endpoint = format!(
        "{host}/{kind}/{workspace_id}/{quiz_id}",
        host = config.server.host,
        kind = EntityKind::QuizRecord,
        workspace_id = quiz_guard.workspace,
        quiz_id = quiz_guard.id,
    );

    rsx! {
        div {
            class: "flex shrink-0 w-full min-h-0 print:hidden p-1",
            ul {
                class: "menu menu-horizontal p-0 m-0 text-base-content flex-nowrap",
                li {
                    button {
                        class: "hover:text-info",
                        onclick: move |event: MouseEvent| {
                            event.prevent_default();
                            event.stop_propagation();
                            document::eval("window.print()");
                        },
                        i { class: "bi bi-printer" }
                        { t!("print") }
                    }
                }
            }
        }
        div {
            class: "flex flex-1 flex-col print-area overflow-y-auto",
            "data-theme": "lofi",
            div {
                class: "flex flex-col w-full items-center gap-1 px-5 pt-4 pb-1",
                div {
                    class: "text-lg font-semibold",
                    "{quiz.read().name}"
                }
                div {
                    class: "",
                    "{quiz.read().path}"
                }
            }
            div {
                class: "tickets-grid",
                for (_, student) in &quiz.read().students {
                    div {
                        class: "ticket",
                        img {
                            class: "rounded-(--radius-box) overflow-hidden",
                            src: QrGenerator::text(format!("{endpoint}/{id}", id = student.id), 150)
                        }
                        if let Some(rank) = &student.rank {
                            div { class: "rank", "{rank}" }
                        }
                        div { class: "", "{student.name}" }
                    }
                }
            }
        }
    }
}
