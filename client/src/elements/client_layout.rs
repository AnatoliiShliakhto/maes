use crate::{elements::*, prelude::*};

#[component]
pub fn ClientLayout() -> Element {
    rsx! {
        div {
            class: "flex-fixed bg-base-200",
            div {
                class: "flex shrink-0 min-h-0 overflow-hidden",
                Header {}
            }
            div {
                class: "flex-fixed p-4",
                // Breadcrumbs {}
                Outlet::<Route> {}
            }
        }
    }
}