use crate::prelude::*;
use ::dioxus::desktop::use_window;

#[derive(Default, Copy, Clone)]
pub struct ContextMenu {
    items: Signal<Vec<ContextMenuItem>>,
    is_visible: Signal<bool>,
    position: Signal<(i32, i32)>,
    raw_position: Signal<(i32, i32)>,
    adjusted_position: Signal<(i32, i32)>,
    needs_measure: Signal<bool>,
}

impl ContextMenu {
    pub fn open(&mut self, evt: MouseEvent, items: Vec<ContextMenuItem>) {
        let x = evt.data.client_coordinates().x as i32;
        let y = evt.data.client_coordinates().y as i32;
        self.raw_position.set((x, y));
        self.position.set((x, y));
        self.adjusted_position.set((x, y));
        self.needs_measure.set(true);
        self.items.set(items);
        self.is_visible.set(true);
    }

    pub fn close(&mut self) {
        self.is_visible.set(false);
    }

    fn len(&self) -> usize {
        self.items.read().len()
    }

    fn needs_measure(&self) -> bool {
        (self.needs_measure)()
    }

    fn is_visible(&self) -> bool {
        (self.is_visible)()
    }

    fn position_x(&self) -> i32 {
        self.position.read().0
    }

    fn position_y(&self) -> i32 {
        self.position.read().1
    }

    fn adjusted_x(&self) -> i32 {
        self.adjusted_position.read().0
    }

    fn adjusted_y(&self) -> i32 {
        self.adjusted_position.read().1
    }

    fn set_adjusted(&mut self, x: i32, y: i32) {
        self.adjusted_position.set((x, y));
        self.position.set((x, y));
        self.needs_measure.set(false);
    }

    fn finish_measure(&mut self) {
        let (x, y) = *self.position.read();
        self.adjusted_position.set((x, y));
        self.needs_measure.set(false);
    }

    #[allow(dead_code)]
    fn raw_x(&self) -> i32 {
        self.raw_position.read().0
    }

    #[allow(dead_code)]
    fn raw_y(&self) -> i32 {
        self.raw_position.read().1
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

    if context_menu.needs_measure() {
        let window = use_window();
        let size = window.inner_size();
        let scale = window.scale_factor();
        let vw = size.width as f64 / scale;
        let vh = size.height as f64 / scale;

        let estimated_w = 280.0_f64.clamp(192.0, 360.0);
        let row_h = 36.0_f64;
        let count = context_menu.len() as f64;
        let mut estimated_h = (count * row_h).min(vh * 0.5);
        if estimated_h < row_h + 8.0 {
            estimated_h = row_h + 8.0;
        }

        let pad = 8.0;

        let mut ax = context_menu.position_x() as f64;
        let mut ay = context_menu.position_y() as f64;

        if ax + estimated_w > vw - pad {
            ax = (vw - estimated_w - pad).max(0.0);
        }

        if ay + estimated_h > vh - pad {
            let up = ay - estimated_h;
            if up >= pad {
                ay = up;
            } else {
                ay = (vh - estimated_h - pad).max(0.0);
            }
        }

        context_menu.set_adjusted(ax as i32, ay as i32);
    }

    rsx! {
        div {
            class: "fixed inset-0 z-[9999]",
            onclick: move |_| context_menu.close(),
            div {
                class: "fixed bg-base-100 border border-base-300 rounded-lg shadow-lg py-1 min-w-48 max-h-[50vh] overflow-auto",
                style: "left: {context_menu.adjusted_x()}px; top: {context_menu.adjusted_y()}px;",

                for (index, item) in items().iter().enumerate() {
                    {
                        let action = item.action;
                        let disabled = item.disabled;
                        let icon = item.icon.clone();
                        let label = item.label.clone();
                        let separator_after = item.separator_after;

                        rsx! {
                            button {
                                key: "ctx_menu_{index}",
                                class: "w-full px-4 py-2 text-left hover:bg-base-200 flex items-center gap-2 text-sm lowercase",
                                class: if disabled { "opacity-50" } else { "cursor-pointer" },
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
                            } else {
                                div { class: "hidden" }
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
        use_callback(move |evt: MouseEvent| {
            evt.stop_propagation();
            evt.prevent_default();
            let items = vec![
                $(
                    $crate::ctx_menu_item!( $($item)+ )
                ),*
            ];
            $crate::components::widgets::use_context_menu().open(evt, items)
        })
    };
}