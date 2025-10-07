use crate::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "flex flex-nowrap shrink-0 w-full bg-base-300 items-center h-14 gap-4 px-2",
            img { src: "/assets/icon.png", class: "size-8"}
            div { class: "flex flex-nowrap text-lg font-semibold overflow-hidden", { t!("app-title") } }
        }
    }
}