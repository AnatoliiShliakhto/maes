use crate::{elements::*, prelude::*};

#[component]
pub fn CleanLayout() -> Element {
    rsx! {
        div {
            class: "flex-fixed bg-base-200",
            Outlet::<Route> {}
        }
    }
}