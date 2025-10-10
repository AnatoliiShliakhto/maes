use crate::prelude::*;

#[component]
pub fn CleanLayout() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}