use crate::{prelude::*, services::*};

#[component]
pub fn SurveyTickets(task: ReadSignal<String>) -> Element {
    let config = ConfigService::read();
    let mut survey = use_signal(SurveyRecord::default);
    let survey_guard = survey.read();

    use_effect(move || {
        api_fetch!(
            GET,
            format!(
                "/api/v1/tasks/{kind}/{id}",
                kind = EntityKind::SurveyRecord,
                id = task.read()
            ),
            on_success = move |body: SurveyRecord| survey.set(body)
        );
    });

    let qr_src = QrGenerator::text(format!(
        "{host}/{kind}/{workspace_id}/{survey_id}",
        host = config.server.host,
        kind = EntityKind::SurveyRecord,
        workspace_id = survey_guard.workspace,
        survey_id = survey_guard.id,
    ), 150);

    rsx! {
        div {
            class: "flex w-full min-h-0 print:hidden p-1",
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
                    "{survey.read().name}"
                }
                div {
                    class: "",
                    "{survey.read().path}"
                }
            }
            div {
                class: "tickets-grid",
                for _ in 0..12 {
                    div {
                        class: "ticket",
                        img {
                            class: "rounded-(--radius-box) overflow-hidden",
                            src: "{qr_src}"
                        }
                        div { class: "", "{survey_guard.name}" }
                    }
                }
            }
        }
    }
}
