use super::panel::*;
use crate::prelude::*;
#[component]
pub fn SplitPanel(
    left_title: Option<String>,
    right_title: Option<String>,
    left_class: Option<String>,
    right_class: Option<String>,
    left: Element,
    right: Element,
) -> Element {
    use_effect(|| {
        document::eval("window.assignSplitter()");
    });

    rsx! {
        div {
            id: "splitter-container",
            class: "grid grow min-h-0 min-w-0 w-full h-full overflow-hidden",
            style: "grid-template-columns: 1fr 10px 1fr;",
            Panel {
                title: left_title,
                class: "mr-0 {left_class.clone().unwrap_or_default()}",
                { left }
            }
            div {
                id: "splitter",
                class: "flex flex-1 cursor-col-resize",
            }
            Panel {
                title: right_title,
                class: "ml-0 {right_class.clone().unwrap_or_default()}",
                { right }
            }
        }
    }
}
