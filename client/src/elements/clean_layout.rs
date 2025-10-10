use crate::prelude::*;

#[component]
pub fn CleanLayout() -> Element {
    use_effect(move || {
        document::eval(r#"
            if (document.documentElement.requestFullscreen) {
                document.documentElement.requestFullscreen();
            } else if (document.documentElement.webkitRequestFullscreen) {
                document.documentElement.webkitRequestFullscreen();
            }
        "#);
    });

    rsx! {
        div {
            class: "flex-fixed bg-base-200",
            Outlet::<Route> {}
        }
    }
}