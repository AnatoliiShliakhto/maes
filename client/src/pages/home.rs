use crate::{prelude::*, utils::*};

#[component]
pub fn Home() -> Element {

    rsx! {
        div {
            class: "flex-scrollable justify-center items-center gap-2",
            div {
                class: "card w-full h-auto max-w-md card-border shadow-lg",
                div {
                    class: "card-body",
                    div {
                        class: "card-title flex text-info text-2xl gap-4 capitalize",
                        i { class: "bi bi-info-circle"}
                        { t!("alert") }
                    }
                    div {
                        class: "h-0.25 bg-base-300",
                    }
                    p { { t!("client-app-announcement") } }
                    div {
                        class: "card-actions justify-end mt-6",
                        button {
                            class: "btn btn-neutral",
                            onclick: move |_| close_window(),
                            { t!("close") }
                        }
                    }
                }
            }
        }
    }
}