use crate::{components::widgets::*, elements::*, prelude::*, services::*};

#[component]
pub fn AppLayout() -> Element {
    let claims = AuthService::claims();

    rsx! {
        div {
            class: "flex flex-1 bg-base-200 min-h-0 overflow-hidden",
            div {
                class: "flex flex-col shrink-0 min-h-0 overflow-y-auto",
                SideMenu {}
            }
            div {
                class: "flex-fixed",
                div {
                    class: "flex flex-nowrap shrink-0 w-full items-center justify-between",
                    Breadcrumbs {}
                    // div {
                    //     class: "text-sm pr-3 text-base-content/60",
                    //     i { class: "bi bi-person-workspace mr-2 text-info/50" }
                    //     "{claims.workspace} | {claims.version}"
                    // }
                }
                Outlet::<Route> {}
            }
        }
    }
}
