#![allow(dead_code)]
#![allow(unused_macros)]
mod elements;
mod common;
mod pages;
mod services;
mod utils;

pub mod prelude {
    pub use crate::{common::*, api_call, api_fetch};
    pub use ::dioxus::prelude::*;
    pub use ::shared::{common::*, models::*, payloads::*, services::*, t};
}

use ::web_sys::wasm_bindgen::prelude::*;
use prelude::*;
use elements::*;

fn main() {
    launch(|| {
        use_future(|| async move {
            loop {
                api_call!(
                    GET,
                    "/health",
                    on_error = move |_| (),
                );
                gloo_timers::future::sleep(std::time::Duration::from_secs(10)).await;
            }
        });


        use_effect(|| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let closure = Closure::wrap(Box::new(move |evt: web_sys::KeyboardEvent| {
                if evt.key_code() == 116 || (evt.ctrl_key() && evt.key_code() == 82) {
                    evt.prevent_default();
                }
            }) as Box<dyn FnMut(_)>);

            document
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();

            closure.forget();
        });

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
        }
    })
}