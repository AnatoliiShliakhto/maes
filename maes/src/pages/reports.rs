use crate::{prelude::*, components::widgets::*};

#[component]
pub fn Reports() -> Element {
    rsx! {
        Panel {
            title: t!("reports"),

        }
    }
}