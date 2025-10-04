use crate::prelude::*;

#[derive(Default, Copy, Clone)]
pub struct ContextMenu {
    is_visible: Signal<bool>,
    position: Signal<(i32, i32)>,
    items: Signal<Vec<ContextMenuItem>>,
}

impl ContextMenu {
    pub fn open(&mut self, evt: MouseEvent, items: Vec<ContextMenuItem>) {
        self.position.set((
            evt.data.client_coordinates().x as i32,
            evt.data.client_coordinates().y as i32,
        ));
        self.items.set(items);
        self.is_visible.set(true);
    }

    pub fn close(&mut self) {
        self.is_visible.set(false);
    }
    
    fn len(&self) -> usize {
        self.items.read().len()
    }
    
    fn position_x(&self) -> i32 {
        self.position.read().0
    }

    fn position_y(&self) -> i32 {
        self.position.read().1
    }
}

pub fn use_init_context_menu() -> ContextMenu {
    use_context_provider(ContextMenu::default)
}

pub fn use_context_menu() -> ContextMenu {
    use_context()
}

#[derive(Clone)]
pub struct ContextMenuItem {
    pub label: String,
    pub icon: Option<String>,
    pub action: Option<Callback>,
    pub disabled: bool,
    pub separator_after: bool,
}

impl ContextMenuItem {
    pub fn new(label: String, action: Option<Callback>) -> Self {
        Self {
            label,
            icon: None,
            action,
            disabled: false,
            separator_after: false,
        }
    }
}

#[component]
pub fn ContextMenuContainer() -> Element {
    let mut context_menu = use_context_menu();
    if !(context_menu.is_visible)() {
        return rsx! {};
    }
    let items = context_menu.items;

    rsx! {
        div {
            class: "fixed inset-0 z-50",
            onclick: move |_| context_menu.close(),
            div {
                class: "fixed bg-base-100 border border-base-300 rounded-lg shadow-lg py-1 min-w-48",
                style: "left: {context_menu.position_x()}px; top: {context_menu.position_y()}px;",

                for (index, item) in items().iter().enumerate() {
                    {
                        let action = item.action;
                        let disabled = item.disabled;
                        let icon = item.icon.clone();
                        let label = item.label.clone();
                        let separator_after = item.separator_after;

                        if disabled { return rsx! { div { key: "ctx_menu_{index}" } } }
                        rsx! {
                            button {
                                key: "ctx_menu_{index}",
                                class: "w-full px-4 py-2 text-left hover:bg-base-200 flex items-center gap-2 text-sm lowercase",
                                class: if disabled { "opacity-50 cursor-not-allowed" } else { "cursor-pointer" },
                                disabled: disabled,
                                onclick: move |_| {
                                    if !disabled {
                                        if let Some(handler) = action {
                                            handler.call(());
                                        }
                                        context_menu.close();
                                    }
                                },
                                if let Some(icon) = &icon {
                                    i { class: "{icon}" }
                                }
                                "{label}"
                            }
                            if separator_after && index < context_menu.len() - 1 {
                                div { class: "border-t border-base-300 my-1" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! ctx_menu_item {
    ($label:expr, $action:expr) => {
        $crate::components::widgets::ContextMenuItem {
            label: $label,
            icon: None,
            action: Some($action),
            disabled: false,
            separator_after: false,
        }
    };
    ($label:expr, $icon:expr, $action:expr) => {
        $crate::components::widgets::ContextMenuItem {
            label: $label,
            icon: Some($icon.to_string()),
            action: Some($action),
            disabled: false,
            separator_after: false,
        }
    };
    ($label:expr, $icon:expr, $action:expr, $disabled:expr) => {
        $crate::components::widgets::ContextMenuItem {
            label: $label,
            icon: Some($icon.to_string()),
            action: Some($action),
            disabled: $disabled,
            separator_after: false,
        }
    };
    ($label:expr, $icon:expr, $action:expr, $disabled:expr, $sep:expr) => {
        $crate::components::widgets::ContextMenuItem {
            label: $label,
            icon: Some($icon.to_string()),
            action: Some($action),
            disabled: $disabled,
            separator_after: $sep,
        }
    };
}

#[macro_export]
macro_rules! make_ctx_menu {
    ([ $( ( $($item:tt)+ ) ),* $(,)? ]) => {
        move |evt: MouseEvent| {
            evt.stop_propagation();
            evt.prevent_default();
            let items = vec![
                $(
                    $crate::ctx_menu_item!( $($item)+ )
                ),*
            ];
            $crate::components::widgets::use_context_menu().open(evt, items)
        }
    };
}
