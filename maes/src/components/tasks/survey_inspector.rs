use super::cards::*;
use crate::{prelude::*, services::*, window::*};
use ::shared::models::*;

#[component]
pub fn SurveyInspector() -> Element {
    let config = ConfigService::read();
    let selected = use_context::<Signal<SelectedItem>>();
    let selected_guard = selected.read();
    let mut pinned = use_signal(|| false);
    let workspace = use_memo(|| AuthService::claims().ws_id.clone());

    let survey_qr = QrGenerator::text(
        format!(
            "{host}/{kind}/{workspace_id}/{survey_id}",
            host = config.server.host,
            kind = EntityKind::SurveyRecord,
            workspace_id = workspace(),
            survey_id = selected_guard.id
        ),
        300,
    );

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
                        div {
                            key: "ticket-{selected_guard.id}",
                            class: "card-body p-3 flex-fixed items-center justify-center gap-2",
                            i { class: "bi bi-incognito text-primary text-4xl" }
                            div { class: "text-2xl font-semibold text-base-content/70", { t!("survey") } }
                            div { class: "font-semibold", "{selected_guard.name}" }
                            div { class: "text-xs text-base-content/60", "{selected_guard.path}" }
                        }
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
            class: "flex-fixed w-full items-center justify-center p-10",
            img {
                class: "max-h-full w-auto object-contain overflow-hidden rounded-(--radius-box) border-1 border-base-200",
                class: "hover:shadow-xl cursor-pointer",
                onclick: move |_| {
                    let selected_guard = selected.read();
                    let url = format!(
                        "{host}/{kind}/{workspace_id}/{survey_id}",
                        host = localhost(),
                        kind = EntityKind::SurveyRecord,
                        workspace_id = workspace(),
                        survey_id = selected_guard.id
                    );
                    WindowManager::open_window(t!("mock-title"), WindowKind::Mock { url })
                },
                src: survey_qr
            }
        }
    }
}
