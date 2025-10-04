use crate::{prelude::*, services::*};
use ::shared::models::*;
use ::std::time::Duration;

#[component]
pub fn QuizInspector() -> Element {
    let kind = use_context::<Signal<EntityKind>>();
    let selected = use_context::<Signal<SelectedItem>>();
    let mut quiz = use_context_provider(|| Signal::new(QuizRecord::default()));
    let _active = use_context_provider(|| Signal::new(QuizRecordStudent::default()));
    let mut search_pattern = use_signal(|| "".to_string());

    use_future(move || async move {
        loop {
            api_fetch!(
                GET,
                format!(
                    "/api/v1/tasks/{kind}/{id}",
                    kind = kind.read(),
                    id = selected.read().id
                ),
                on_success = move |body: QuizRecord| quiz.set(body),
            );
            tokio::time::sleep(Duration::from_secs(5)).await
        }
    });

    rsx! {
        div {
            class: "flex w-full h-50 shrink-0 group [perspective:1000px] px-1 cursor-pointer",
            div {
                class: "relative h-full w-full",
                class: "transition-all duration-500 [transform-style:preserve-3d] group-hover:[transform:rotateY(180deg)]",

                div {
                    class: "absolute inset-0",
                    i { class: "bi bi-arrow-repeat text-base-content/50 absolute top-2 right-3" }
                    div {
                        class: "card h-full w-full",
                        div {
                            class: "card-body p-3",
                            RenderTicketCard {}
                        }
                    }
                }

                div {
                    class: "absolute inset-0 h-full w-full rounded-xl bg-base-100 text-base-content",
                    class: "[transform:rotateY(180deg)] [backface-visibility:hidden]",
                    i { class: "bi bi-arrow-repeat text-base-content/50 absolute top-2 left-3" }
                    div {
                        class: "card h-full w-full",
                        div {
                            class: "card-body p-3",
                            RenderWifiCard {}
                        }
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
                i { class: "bi bi-search relative -right-0" }
            }
        }
        div {
            class: "flex-scrollable",
            ul {
                class: "list w-full",
                {
                    let pat = search_pattern.read().to_lowercase();
                    rsx! {
                        for s in quiz.read().students.values().filter(|s| s.name.to_lowercase().contains(&pat)) {
                            RenderStudentItem { key: "{s.id}", student: s.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderTicketCard() -> Element {
    let config = ConfigService::read();
    let quiz = use_context::<Signal<QuizRecord>>();
    let active = use_context::<Signal<QuizRecordStudent>>();
    let quiz_guard = quiz.read();
    let active_guard = active.read();

    rsx! {
        div {
            key: "ticket-{active_guard.id}",
            class: "flex-fixed items-center justify-center gap-2",
            if active_guard.id.is_empty() {
                i { class: "bi bi-mortarboard text-primary text-4xl" }
                div {
                    class: "text-2xl font-semibold text-base-content/70",
                    { t!("quiz") }
                }
                div {
                    class: "font-semibold",
                    "{quiz_guard.name}"
                }
                div {
                    class: "text-xs text-base-content/60",
                    "{quiz_guard.path}"
                }
            } else {
                div {
                    class: "flex flex-1 w-full gap-2",
                    div {
                        class: "h-full max-h-44",
                        img {
                            class: "max-h-full w-auto object-contain overflow-hidden rounded-(--radius-box)",
                            src: QrGenerator::text(
                                format!("{host}/task/{kind}/{quiz_id}/{id}",
                                    host = config.server.host,
                                    kind = EntityKind::QuizRecord,
                                    quiz_id = quiz_guard.id,
                                    id = active_guard.id
                                ),
                                300
                            )
                        }
                    }
                    div {
                        class: "flex-fixed items-center justify-center gap-2",
                        div {
                            class: "text-2xl font-semibold text-base-content/70",
                            { t!("ticket") }
                        }
                        if let Some(rank) = &active_guard.rank {
                            div {
                                class: "text-xs text-base-content/60",
                                "{rank}"
                            }
                        }
                        div {
                            class: "font-semibold",
                            "{active_guard.name}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn RenderWifiCard() -> Element {
    let config = ConfigService::read();
    let wifi_payload = format!(
        "WIFI:S:{ssid};T:WPA;P:{password};;",
        ssid = config.wifi.ssid,
        password = config.wifi.password
    );

    rsx! {
        div {
            class: "flex flex-1 w-full gap-2",
            div {
                class: "flex-fixed items-center justify-center gap-2",
                span { class: "text-base-content/70 text-2xl font-semibold", { t!("wifi") } }
                // i { class: "bi bi-router text-base-content/70 text-4xl" }
                div {
                    class: "flex flex-col text-lg",
                    div {
                        class: "flex flex-nowrap gap-3",
                        i { class: "bi bi-wifi text-primary" }
                        "{config.wifi.ssid}"
                    }
                    div {
                        class: "flex flex-nowrap gap-3",
                        i { class: "bi bi-key text-primary" }
                        "{config.wifi.password}"
                    }
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
fn RenderStudentItem(student: ReadOnlySignal<QuizRecordStudent>) -> Element {
    let student_guard = student.read();
    let mut active = use_context::<Signal<QuizRecordStudent>>();

    rsx! {
        li {
            class: format!("list-row rounded-none px-4 py-0 cursor-pointer hover:bg-base-200 {class}", class =
                if student_guard.id == active.read().id { "bg-base-300" } else { "" }),
            onclick: move |_| active.set(student()),
            div {
                class: "list-col-grow flex flex-col justify-center my-3 gap-1",
                div {
                    class: "font-semibold",
                    "{student_guard.name}"
                }
                div {
                    class: "text-xs text-base-content/60",
                    if let Some(rank) = student_guard.rank.clone() {
                        "{rank}"
                    }
                }
            }
        }
    }
}
