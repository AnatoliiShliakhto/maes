use crate::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "navbar bg-base-100 shadow-sm min-h-0 p-0",
            div {
                class: "flex-none",
                label {
                    r#for: "app-drawer",
                    class: "btn btn-lg drawer-button btn-ghost rounded-none lg:hidden text-2xl",
                    i { class: "bi bi-list" }
                }
            }
            div {
                class: "flex flex-1 items-center justify-center px-2",
                span {
                    class: "text-2xl font-semibold",
                    "Aide"
                }
            }
            div {
                class: "flex-none",
                button {
                    class: "btn btn-lg btn-ghost text-2xl rounded-none",
                    i { class: "bi bi-three-dots-vertical" }
                }
            }
        }
    }
}