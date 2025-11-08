#![allow(dead_code)]
#![allow(unused_macros)]
mod elements;
mod common;
mod pages;
mod services;
mod components;

pub mod prelude {
    pub use crate::{common::*};
    pub use ::dioxus::prelude::*;
    pub use ::shared::{models::*, payloads::*, services::*, t};
}

use prelude::*;
use elements::*;
use components::dialogs::*;

fn main() {
    launch(|| {
        let _dialog = use_init_dialog();

        rsx! {
            div {
                class: "flex-fixed h-dvh w-dvw min-h-screen",
                oncontextmenu: move |evt| {
                    if !cfg!(debug_assertions) {
                        evt.prevent_default();
                    }
                },
                Head {}
                Router::<Route> {}
            }
            DialogContainer { key: "dialog-container" }
        }
    })
}
