use crate::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx! {
        div {
            class: "flex flex-1 items-center justify-center",
            div {
                class: "inline-flex items-center gap-6 text-slate-600",
                span {
                    class: "loading loading-bars size-12"
                }
                span {
                    class: "text-2xl font-semibold",
                    { t!("loading") }
                }
            }
        }
    }
}
