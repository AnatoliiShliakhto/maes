use crate::prelude::*;

#[component]
pub fn ProgressCircle(progress: ReadSignal<usize>) -> Element {
    rsx! {
        match progress() {
            0 => rsx! {
                div {
                    class: "flex items-center justify-center",
                    i { class: "bi bi-hourglass text-base-content/70 text-xl" }
                }
            },
            1..=99 => rsx! {
                div {
                    role: "progressbar",
                    class: match progress() {
                        1..=25 => "radial-progress text-error text-xs",
                        26..=50 => "radial-progress text-warning text-xs",
                        51..=75 => "radial-progress text-info text-xs",
                        76..=100 => "radial-progress text-success text-xs",
                        _ => "radial-progress text-base-content/70 text-xs",
                    },
                    style: format!("--value:{progress}; --size:2rem; --thickness: 0.2rem;"),
                    aria_valuenow: "{progress}",
                    div {
                        class: "text-base-content/60",
                        "{progress}%"
                    }
                }
            },
            _ => rsx! {
                div {
                    class: "flex items-center justify-center",
                    i { class: "bi bi-check-all text-success text-xl" }
                }
            }
        }
    }
}