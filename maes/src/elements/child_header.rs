use crate::{prelude::*, services::*};
use ::dioxus::desktop::use_window;

#[component]
pub fn ChildHeader(title: ReadSignal<String>) -> Element {
    let mut pinned = use_signal(|| false);

    let close_window = move |_| {
        let window = use_window();
        let scale = use_window().window.scale_factor();
        ConfigService::with_mut(|config| {
            if window.is_maximized() {
                config.windows.child.maximized = true;
            } else {
                if let Ok(position) = window.outer_position() {
                    config.windows.child.left = (position.x as f64 / scale) as i32;
                    config.windows.child.top = (position.y as f64 / scale) as i32;
                }
                let size = window.inner_size();
                config.windows.child.width = (size.width as f64 / scale) as i32;
                config.windows.child.height = (size.height as f64 / scale) as i32;
                config.windows.child.maximized = false;
            }
        })
            .ok();
        window.close();
    };

    rsx! {
        nav {
            class: "navbar flex shrink-0 w-full bg-neutral text-neutral-content min-h-0 p-0 items-center print:hidden",
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
                    "{title}"
                }
            }
            div {
                if cfg!(debug_assertions) {
                    button {
                        class: "btn btn-sm btn-square btn-ghost rounded-none hover:btn-secondary",
                        onclick: move |_| use_window().devtool(),
                        i { class: "bi bi-bug" }
                    }
                }
                button {
                    class: "btn btn-sm btn-square btn-ghost rounded-none hover:btn-secondary",
                    onclick: move |_| use_window().set_minimized(true),
                    i { class: "bi bi-dash" }
                }
                button {
                    class: "btn btn-sm btn-square btn-ghost rounded-none hover:btn-secondary",
                    onclick: move |_| {
                        let window = use_window();
                        window.set_maximized(!window.is_maximized())
                    },
                    i { class: "bi bi-arrows-angle-expand" }
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
