use crate::{elements::*, prelude::*};

#[component]
pub fn ClientLayout() -> Element {
    use_effect(move || {
        document::eval(r#"
            if (document.fullscreenElement) {
                document.exitFullscreen();
            } else if (document.webkitFullscreenElement) {
                document.webkitExitFullscreen();
            }
        "#);
    });

    rsx! {
        div {
            class: "flex-fixed bg-base-200",
            div {
                class: "flex shrink-0 min-h-0 overflow-hidden",
                Header {}
            }
            div {
                class: "flex-fixed",
                Outlet::<Route> {}
            }
        }
    }
}