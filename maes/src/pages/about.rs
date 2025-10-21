use ::chrono::{Local, Datelike};
use crate::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "flex gap-4 w-full bg-base-200",
            img {
                class: "w-50",
                src: "/assets/ok.svg",
            }
            div {
                class: "flex flex-col items-start justify-center gap-2 text-lg font-semibold",
                span { { t!("about-title-1") } }
                span { { t!("about-title-2") } }
                span { { t!("about-title-3") } }
                span { { t!("about-title-4") } }
            }
        }
        div {
            class: "flex-fixed items-center justify-center",
            div {
                class: "flex w-full font-bold text-4xl items-center justify-center mt-5 text-primary",
                { t!("about-title-version", version = env!("CARGO_PKG_VERSION")) }
            }
            div {
                class: "flex w-full font-semibold text-xl items-center justify-center mt-2 text-base-content/70",
                { t!("about-title") }
            }
            div {
                class: "flex w-full font-semibold text-xl items-center justify-center mt-0 text-base-content/70",
                { t!("about-description") }
            }
            div {
                class: "flex w-full justify-center items-center pt-10",
                a {
                    class: "btn btn-xl",
                    href: "https://github.com/AnatoliiShliakhto/maes",
                    target: "_blank",
                    i { class: "bi bi-github mr-2" }
                    { t!("about-site")}
                }
            }
        }
        div {
            class: "flex shrink-0 w-full items-end justify-center mb-5 text-base-content/60",
            { t!("about-copyright", year = Local::now().year()) }
        }
    }
}
