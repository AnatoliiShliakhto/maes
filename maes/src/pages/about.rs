use crate::{prelude::*, components::widgets::*};

#[component]
pub fn About() -> Element {
    rsx! {
        Panel {
            title: t!("about"),
            
        }
    }
}