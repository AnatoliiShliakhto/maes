use crate::prelude::*;

#[component]
pub fn Rating(grade: usize) -> Element {
    let class = match grade {
        5 => "mask mask-star bg-success",
        4 => "mask mask-star bg-info",
        3 => "mask mask-star bg-warning",
        _ => "mask mask-star bg-error",
    };

    if grade == 0 {
        return rsx! {}
    }

    rsx! {
        div {
            class: "rating rating-xs",
            div { class: "{class}" }
            if grade == 2 {
                div { class: "{class}", aria_current: true }
            } else {
                div { class: "{class}" }
            }
            if grade == 3 {
                div { class: "{class}", aria_current: true }
            } else {
                div { class: "{class}" }
            }
            if grade == 4 {
                div { class: "{class}", aria_current: true }
            } else {
                div { class: "{class}" }
            }
            if grade == 5 {
                div { class: "{class}", aria_current: true }
            } else {
                div { class: "{class}" }
            }
        }
    }
}
