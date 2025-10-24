use crate::{prelude::*, services::*};

#[component]
pub fn SideMenu() -> Element {
    let claims = AuthService::claims();

    rsx! {
        ul {
            class: "menu flex-wrap pl-2 m-0 mt-2 group transition-all duration-300 ease-in-out whitespace-nowrap",
            class: "w-15 hover:w-52 overflow-hidden",
            HotspotMenuItem {}
            // MenuItem {
            //     to: Route::Tasks {},
            //     icon: rsx! { i { class: "bi bi-qr-code text-2xl" } },
            //     label: t!("wifi-qr-code")
            // }
            li { class: "mx-0" }
            MenuItem {
                to: Route::Tasks {},
                icon: rsx! { i { class: "bi bi-activity text-2xl" } },
                label: t!("tasks")
            }
            MenuItem {
                to: Route::Reports {},
                icon: rsx! { i { class: "bi bi-file-earmark-text text-2xl" } },
                label: t!("reports")
            }
            MenuItem {
                to: Route::Students {},
                icon: rsx! { i { class: "bi bi-people text-2xl" } },
                label: t!("students")
            }
            if claims.is_supervisor() {
                li { class: "mx-0" }
                MenuItem {
                    to: Route::WorkspaceManager {},
                    icon: rsx! { i { class: "bi bi-person-workspace text-2xl" } },
                    label: t!("workspace")
                }
                MenuItem {
                    to: Route::WorkspaceQuizzes {},
                    icon: rsx! { i { class: "bi bi-mortarboard text-2xl" } },
                    label: t!("quizzes")
                }
                MenuItem {
                    to: Route::WorkspaceSurveys {},
                    icon: rsx! { i { class: "bi bi-incognito text-2xl" } },
                    label: t!("surveys")
                }
            }
            li { class: "mx-0" }
            // if claims.is_admin() {
            //     MenuItem {
            //         icon: rsx! { i { class: "bi bi-box-seam text-2xl" } },
            //         label: t!("export"),
            //         onclick: move |_| Exchange::export(vec![])
            //     }
            // }
            MenuItem {
                to: Route::Settings {},
                icon: rsx! { i { class: "bi bi-gear text-2xl" } },
                label: t!("settings")
            }
            MenuItem {
                icon: rsx! { i { class: "bi bi-box-arrow-right text-2xl" } },
                label: t!("sign-out"),
                onclick: |_| AuthService::logout(),
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct MenuItemProps {
    to: Option<Route>,
    icon: Option<Element>,
    label: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
fn MenuItem(props: MenuItemProps) -> Element {
    rsx! {
        li {
            if let Some(to) = props.to {
                Link {
                    class: if to == use_route::<Route>() {
                        "flex flex-nowrap bg-accent/30 text-secondary w-12 group-hover:w-auto"
                    } else {
                        "flex flex-nowrap w-12 group-hover:w-auto"
                    },
                    to,
                    onclick: props.onclick,
                    if let Some(icon) = props.icon { { icon } }
                    if let Some(label) = props.label {
                        span {
                            class: "pl-1 inline-block whitespace-nowrap overflow-hidden",
                            class: "opacity-0 group-hover:opacity-100",
                            "{label}"
                        }
                    }
                }
            } else {
                a {
                    class: "flex flex-nowrap w-12 group-hover:w-auto",
                    onclick: move |evt| {
                        if let Some(cb) = props.onclick.as_ref() {
                            cb.call(evt);
                        }
                    },
                    if let Some(icon) = props.icon { { icon } }
                    if let Some(label) = props.label {
                        span {
                            class: "px-1 inline-block whitespace-nowrap overflow-hidden",
                            class: "opacity-0 group-hover:opacity-100",
                            "{label}" }
                    }
                }
            }
        }
    }
}

#[component]
fn HotspotMenuItem() -> Element {
    let status = HotspotService::use_init_status();
    rsx!{
        li {
            if *status.read() {
                a {
                    class: "flex flex-nowrap w-12 group-hover:w-auto",
                    onclick: move |_| HotspotService::stop(),
                    div {
                        class: "indicator",
                        span { class: "indicator-item indicator-bottom status status-success bottom-[8px]" }
                        i { class: "bi bi-wifi text-2xl text-success" }
                    }
                    span {
                        class: "pl-1 inline-block whitespace-nowrap overflow-hidden",
                        class: "opacity-0 group-hover:opacity-100",
                        { t!("wifi-ap-active") }
                    }
                }
            } else {
                a {
                    class: "flex flex-nowrap w-12 group-hover:w-auto",
                    onclick: move |_| {
                        let config = ConfigService::read();
                        HotspotService::start(&config.wifi.ssid, &config.wifi.password, config.wifi.direct);
                    },
                    div {
                        class: "indicator",
                        span { class: "indicator-item indicator-bottom status status-error bottom-[8px]" }
                        i { class: "bi bi-wifi-off text-2xl text-error" }
                    }
                    span {
                        class: "pl-1 inline-block whitespace-nowrap overflow-hidden",
                        class: "opacity-0 group-hover:opacity-100",
                        { t!("wifi-ap-inactive") }
                    }
                }
            }
        }
    }
}