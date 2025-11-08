use crate::prelude::*;

#[component]
pub fn CleanLayout() -> Element {
    rsx! {
        div {
            class: "flex-fixed",
                Outlet::<Route> {}
        }
    }
}
