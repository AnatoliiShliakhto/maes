use crate::{prelude::*, services::*, utils::*};

#[component]
pub fn ErrorPage() -> Element {
    let navigator = use_navigator();
    let mut message = ErrorService::message();

    rsx! {
        div {
            class: "flex-scrollable justify-center items-center gap-2 p-4",
            div {
                class: "card w-full h-auto max-w-md card-border shadow-lg bg-base-100",
                div {
                    class: "card-body",
                    div {
                        class: "card-title flex text-error text-2xl gap-4 capitalize",
                        i { class: "bi bi-x-circle-fill"}
                        { t!("error") }
                    }
                    div {
                        class: "h-0.25 bg-base-300",
                    }
                    p {
                        if message.read().is_empty() { { t!("something-went-wrong") } } else { "{message}" }
                    }
                    div {
                        class: "card-actions justify-end mt-6",
                        if navigator.can_go_back() {
                            button {
                                class: "btn btn-neutral",
                                onclick: move |_| {
                                    message.set(String::new());
                                    navigator.go_back()
                                },
                                { t!("try-again") }
                            }
                        }
                    }
                }
            }
            div {
                class: "flex",
                button {
                    class: "btn btn-link text-base-content/60 lowercase",
                    onclick: move |_| close_window(),
                    { t!("close") }
                }
            }
        }
    }
}