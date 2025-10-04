use crate::prelude::*;

#[component]
pub fn TaskList() -> Element {

    rsx! {
        div {
            class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 items-center h-10 space-between",
            i { class: "bi bi-three-dots-vertical" }
            div {
                class: "w-full",
                { t!("tasks") }
            }
            ul {
                class: "menu menu-horizontal p-0 m-0 text-base-content flex-nowrap",
                li {
                    button {
                        class: "hover:text-success",
                        i { class: "bi bi-plus" }
                    }
                }
            }
        }
        div {
            class: "h-0.25 bg-base-300 mx-4 my-1",
        }

        ul {
            class: "list w-full overflow-y-auto",
            //todo
        }
    }        
}
