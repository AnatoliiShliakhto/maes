use crate::{prelude::*, services::*};
use ::shared::models::*;

#[component]
pub fn RenderWifiCard() -> Element {
    let config = ConfigService::read();
    let wifi_payload = format!(
        "WIFI:S:{};T:WPA;P:{};;",
        config.wifi.ssid, config.wifi.password
    );

    rsx! {
        div {
            class: "flex flex-1 w-full gap-2",
            div {
                class: "flex-fixed items-center justify-center gap-2",
                span { class: "text-base-content/70 text-2xl font-semibold", { t!("wifi") } }
                div {
                    class: "flex flex-col text-lg",
                    div { class: "flex flex-nowrap gap-3", i { class: "bi bi-wifi text-primary" } "{config.wifi.ssid}" }
                    div { class: "flex flex-nowrap gap-3", i { class: "bi bi-key text-primary" } "{config.wifi.password}" }
                }
            }
            div {
                class: "h-full max-h-44",
                img {
                    class: "max-h-full w-auto object-contain overflow-hidden rounded-(--radius-box)",
                    src: QrGenerator::text(wifi_payload, 300)
                }
            }
        }
    }
}

#[component]
pub fn RenderTicketCard() -> Element {
    let config = ConfigService::read();
    let quiz = use_context::<Signal<QuizRecord>>();
    let active = use_context::<Signal<QuizRecordStudent>>();
    let quiz_guard = quiz.read();
    let active_guard = active.read();

    let qr_src = if !active_guard.id.is_empty() {
        QrGenerator::text(
            format!(
                "{host}/{kind}/{workspace_id}/{quiz_id}/{student_id}",
                host = config.server.host,
                kind = EntityKind::QuizRecord,
                workspace_id = quiz_guard.workspace,
                quiz_id = quiz_guard.id,
                student_id = active_guard.id
            ),
            300,
        )
    } else {
        String::new()
    };

    rsx! {
        div {
            key: "ticket-{active_guard.id}",
            class: "flex-fixed items-center justify-center gap-2",
            if active_guard.id.is_empty() {
                i { class: "bi bi-mortarboard text-primary text-4xl" }
                div { class: "text-2xl font-semibold text-base-content/70", { t!("quiz") } }
                div { class: "font-semibold", "{quiz_guard.name}" }
                div { class: "text-xs text-base-content/60", "{quiz_guard.path}" }
            } else {
                div {
                    class: "flex flex-1 w-full gap-2",
                    div {
                        class: "h-full max-h-44",
                        img {
                            class: "max-h-full w-auto object-contain overflow-hidden rounded-(--radius-box)",
                            src: qr_src
                        }
                    }
                    div {
                        class: "flex-fixed items-center justify-center gap-2",
                        div { class: "text-2xl font-semibold text-base-content/70", { t!("ticket") } }
                        if let Some(rank) = &active_guard.rank {
                            div { class: "text-xs text-base-content/60", "{rank}" }
                        }
                        div { class: "font-semibold", "{active_guard.name}" }
                    }
                }
            }
        }
    }
}
