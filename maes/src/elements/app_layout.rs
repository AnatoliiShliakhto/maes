use crate::{components::widgets::*, elements::*, prelude::*};

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div {
            class: "flex flex-1 bg-base-200 min-h-0 overflow-hidden",
            div {
                class: "flex shrink-0 min-h-0 overflow-y-auto",
                SideMenu {}
            }
            div {
                class: "flex-fixed",
                Breadcrumbs {}
                Outlet::<Route> {}
            }
        }
    }
}
