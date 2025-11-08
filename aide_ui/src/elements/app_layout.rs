use crate::{prelude::*, elements::*};

#[component]
pub fn AppLayout() -> Element {
    let navigator = use_navigator();

    rsx! {
        div {
            class: "drawer lg:drawer-open",
            input {
                id: "app-drawer",
                r#type: "checkbox",
                class: "drawer-toggle",
            }
            div {
                class: "drawer-content flex-fixed",
                Header {}
                Outlet::<Route> {}
                div {
                    class: "dock",
                    button {
                        class: if use_route::<Route>() == (Route::Home {}) { "dock-active" } else { "dock" },
                        i { class: "bi bi-house" }
                        // span {
                        //     class: "dock-label",
                        //     "Home"
                        // }
                    }
                    button {
                        class: if use_route::<Route>() == (Route::QrScanner {}) { "dock-active" } else { "dock" },
                        onclick: move |_| { _ = navigator.push(Route::QrScanner {}); },
                        i { class: "bi bi-qr-code-scan" }
                        // span {
                        //     class: "dock-label",
                        //     "Home"
                        // }
                    }
                }
            }
            div {
                class: "drawer-side",
                label {
                    r#for: "app-drawer",
                    class: "drawer-overlay",
                    aria_label: "Close"
                }
                ul {
                    class: "menu bg-base-200 min-h-full w-full sm:w-80 p-4",
                    li {
                        a { "First item" }
                    }
                }
            }
        }        
    }
}