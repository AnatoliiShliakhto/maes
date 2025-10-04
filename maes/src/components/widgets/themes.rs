use crate::{prelude::*, services::*};
use ::std::ops::Deref;

static THEMES: [&str; 31] = [
    "light",
    "dark",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
    "synthwave",
    "retro",
    "garden",
    "forest",
    "aqua",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "black",
    "luxury",
    "dracula",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "night",
    "coffee",
    "winter",
    "dim",
    "nord",
    "sunset",
    "abyss",
    "silk",
];

#[component]
pub fn Themes() -> Element {
    if THEMES.len() < 2 {
        return rsx!();
    }

    let mut active_theme = use_signal(|| ConfigService::read().theme.clone());

    let change_theme = move |evt: FormEvent| {
        let theme = evt.value();
        if !theme.is_empty() {
            ConfigService::with_mut(|config| config.theme = theme.clone()).ok();
            active_theme.set(theme)
        }
    };

    rsx! {
        div {
            class: "dropdown dropdown-end block",
            title: t!("theme-change"),
            button {
                class: "btn btn-square btn-ghost rounded-none hover:btn-secondary",
                i { class: "bi bi-palette2" }
            }
            ul {
                class: "dropdown-content bg-base-200 text-base-content rounded-(--radius-box)",
                class: "max-h-100 w-46 mt-0.5 overflow-y-auto",
                class: "border border-white/5 shadow-2xl outline-1 outline-black/5 z-100",
                tabindex: 0,
                form {
                    onchange: change_theme,
                    for theme in THEMES.iter().map(<&str>::deref) {
                        li {
                            div {
                                class: "bg-base-100 grid shrink-0 grid-cols-2 gap-0.5 rounded-(--radius-selector) p-1 shadow-sm",
                                class: "fixed mt-2.75 right-3 z-10",
                                "data-theme": theme,
                                div { class: "bg-base-content size-1 rounded-full" }
                                div { class: "bg-primary size-1 rounded-full" }
                                div { class: "bg-secondary size-1 rounded-full" }
                                div { class: "bg-accent size-1 rounded-full" }
                            }
                            input {
                                class: "theme-controller btn btn-block btn-ghost justify-start",
                                r#type: "radio",
                                name: "theme-dropdown",
                                value: theme,
                                initial_checked: active_theme().eq(theme),
                                aria_label: t!(format!("theme-{theme}")),
                            }
                        }
                    }
                }
            }
        }
    }
}
