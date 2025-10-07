use crate::{prelude::*, services::*};
use ::dioxus::desktop::use_window;

#[component]
pub fn MockHeader() -> Element {
    let mut pinned = use_signal(|| false);

    let close_window = move |_| {
        let window = use_window();
        let scale = use_window().window.scale_factor();
        ConfigService::with_mut(|config| {
            if let Ok(position) = window.outer_position() {
                config.windows.mock.left = (position.x as f64 / scale) as i32;
                config.windows.mock.top = (position.y as f64 / scale) as i32;
            }
            let size = window.inner_size();
            config.windows.mock.width = (size.width as f64 / scale) as i32;
            config.windows.mock.height = (size.height as f64 / scale) as i32;
        })
        .ok();
        window.close();
    };

    rsx! {
        nav {
            class: "navbar bg-base-300 text-neutral-content min-h-0 p-0 items-center",
            button {
                class: "btn btn-sm btn-ghost btn-square rounded-none",
                onclick: move |_| {
                    pinned.set(!pinned());
                    use_window().set_always_on_top(pinned());
                },
                if pinned() {
                    i { class: "bi bi-pin text-accent" }
                } else {
                    i { class: "bi bi-pin-angle" }
                }

            }
            div {
                class: "flex flex-1 flex-nowrap gap-4 px-1 justify-start cursor-move",
                onmousedown: move |_| use_window().drag(),
                span {
                    class: "font-normal text-sm",
                    { t!("mock-title") }
                }
            }
            div {
                button {
                    class: "btn btn-sm btn-square btn-ghost rounded-none hover:btn-secondary",
                    onclick: move |_| use_window().set_minimized(true),
                    i { class: "bi bi-dash" }
                }
                button {
                    class: "btn btn-sm btn-square btn-ghost hover:btn-error rounded-none",
                    onclick: close_window,
                    i { class: "bi bi-x-lg" }
                }
            }
        }
    }
}
