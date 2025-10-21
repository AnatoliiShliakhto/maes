use crate::{components::widgets::*, prelude::*, services::*, window::*};
use ::dioxus::desktop::use_window;

#[component]
pub fn AppHeader() -> Element {
    let claims = AuthService::claims();

    let close_window = move |_| {
        let window = use_window();
        let scale = use_window().window.scale_factor();
        ConfigService::with_mut(|config| {
            if window.is_maximized() {
                config.windows.main.maximized = true;
            } else {
                if let Ok(position) = window.outer_position() {
                    config.windows.main.left = (position.x as f64 / scale) as i32;
                    config.windows.main.top = (position.y as f64 / scale) as i32;
                }
                let size = window.inner_size();
                config.windows.main.width = (size.width as f64 / scale) as i32;
                config.windows.main.height = (size.height as f64 / scale) as i32;
                config.windows.main.maximized = false;
            }
        })
        .ok();
        window.close();
    };

    rsx! {
        nav {
            class: "navbar bg-neutral text-neutral-content min-h-0 p-0 items-center",
            div {
                role: "button",
                class: "btn btn-ghost btn-square rounded-none hover:btn-secondary p-0",
                onclick: move |_| WindowManager::open_window(t!("about"), WindowKind::About),
                //i { class: "bi bi-three-dots-vertical text-xl" }
                img { src: "/assets/icon.png", class: "w-6"}
            }
            div {
                class: "flex flex-1 flex-nowrap gap-4 px-1 justify-start cursor-move",
                onmousedown: move |_| use_window().drag(),
                span {
                    class: "font-normal",
                    if claims.is_authorized() {
                        { t!("app-workspace-title", username = claims.username.clone(), version = claims.version.clone()) }
                    } else {
                        { t!("app-title") }
                    }
                }
            }
            div {
                class: "flex shrink-0 flex-nowrap",
                if cfg!(debug_assertions) {
                    button {
                        class: "btn btn-square btn-ghost rounded-none hover:btn-secondary",
                        onclick: move |_| use_window().devtool(),
                        i { class: "bi bi-bug" }
                    }
                }
                Themes {}
                button {
                    class: "btn btn-square btn-ghost rounded-none hover:btn-secondary",
                    onclick: move |_| use_window().set_minimized(true),
                    i { class: "bi bi-dash" }
                }
                button {
                    class: "btn btn-square btn-ghost rounded-none hover:btn-secondary",
                    onclick: move |_| {
                        let window = use_window();
                        window.set_maximized(!window.is_maximized())
                    },
                    i { class: "bi bi-arrows-angle-expand" }
                }
                button {
                    class: "btn btn-square btn-ghost hover:btn-error rounded-none",
                    onclick: close_window,
                    i { class: "bi bi-x-lg" }
                }
            }
        }
    }
}
