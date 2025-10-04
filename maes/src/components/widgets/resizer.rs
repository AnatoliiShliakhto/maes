use ::dioxus::{
    desktop::{tao::window::ResizeDirection, use_window},
    prelude::*,
};

#[component]
pub fn WindowResizer() -> Element {
    let window = use_window();

    rsx! {
        div {
            class: "absolute bottom-0 right-0 w-4 h-4 cursor-se-resize",
            onmousedown: move |event| { 
                window.drag_resize_window(ResizeDirection::SouthEast).ok(); 
            },
        }
    }
}
