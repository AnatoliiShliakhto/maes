use crate::prelude::*;

#[component]
pub fn Panel(title: Option<String>, class: Option<String>, children: Element) -> Element {
    let class = class.unwrap_or_default();

    rsx! {
        div {
            class: "flex-fixed rounded-(--radius-box) shadow mx-2 mb-2 bg-base-100 {class}",
            if let Some(title) = title {
                div {
                    class: "flex shrink-0 w-full gap-2 px-3 pt-3",
                    i { class: "bi bi-list" }
                    "{title}"
                }
                div {
                    class: "h-0.25 bg-base-300 mx-4 my-2",
                }
            }
            div {
                class: "flex-fixed",
                { children }
            }
        }
    }
}