#![allow(dead_code)]
use crate::prelude::*;

#[derive(Default, Clone, PartialEq)]
pub enum DialogType {
    #[default]
    Alert,
    Success,
    Error,
    Warning,
    Info,
}

impl DialogType {
    fn title(&self) -> &'static str {
        match self {
            Self::Alert => "alert",
            Self::Success => "success",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }

    fn css_class(&self) -> &'static str {
        match self {
            Self::Alert => "text-accent",
            Self::Success => "text-success",
            Self::Error => "text-error",
            Self::Warning => "text-warning",
            Self::Info => "text-info",
        }
    }

    fn css_btn_class(&self) -> &'static str {
        match self {
            Self::Alert => "btn-primary",
            Self::Success => "btn-success",
            Self::Error => "btn-error",
            Self::Warning => "btn-warning",
            Self::Info => "btn-info",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::Alert => "bi bi-exclamation-triangle",
            Self::Success => "bi bi-check-circle-fill",
            Self::Error => "bi bi-x-circle-fill",
            Self::Warning => "bi bi-exclamation-triangle",
            Self::Info => "bi bi-info-circle-fill",
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Dialog {
    pub is_visible: Signal<bool>,
    r#type: Signal<DialogType>,
    message: Signal<Option<String>>,
    on_confirm: Signal<Option<Callback>>,
}

impl Dialog {
    pub fn open(&mut self, r#type: DialogType, message: String, on_confirm: Option<Callback>) {
        self.r#type.set(r#type);
        self.message.set(Some(message));
        self.on_confirm.set(on_confirm);
        self.is_visible.set(true);
    }

    pub fn close(&mut self) {
        self.message.set(None);
        self.on_confirm.set(None);
        self.is_visible.set(false);
    }

    pub fn alert(&mut self, message: impl Into<String>, on_confirm: Option<Callback>) {
        self.open(DialogType::Alert, message.into(), on_confirm);
    }

    pub fn success(&mut self, message: impl Into<String>, on_confirm: Option<Callback>) {
        self.open(DialogType::Success, message.into(), on_confirm);
    }

    pub fn error(&mut self, message: impl Into<String>, on_confirm: Option<Callback>) {
        self.open(DialogType::Error, message.into(), on_confirm);
    }

    pub fn warning(&mut self, message: impl Into<String>, on_confirm: Option<Callback>) {
        self.open(DialogType::Warning, message.into(), on_confirm);
    }

    pub fn info(&mut self, message: impl Into<String>, on_confirm: Option<Callback>) {
        self.open(DialogType::Info, message.into(), on_confirm);
    }
}

pub fn use_init_dialog() -> Dialog {
    use_context_provider(Dialog::default)
}

pub fn use_dialog() -> Dialog {
    use_context()
}

#[component]
pub fn DialogContainer() -> Element {
    let mut dialog = use_dialog();
    if !(dialog.is_visible)() {
        return rsx! {};
    };

    rsx! {
        dialog {
            class: "modal modal-open",
            // onclick: move |_| dialog_args().hide(),
            div {
                class: "modal-box flex flex-col gap-5",
                onclick: |evt| evt.stop_propagation(),
                h3 {
                    class: "text-lg semibold {dialog.r#type.read().css_class()}",
                    i { class: "{dialog.r#type.read().icon()} mr-3 text-2xl" }
                    { t!((dialog.r#type)().title()) }
                }
                p {
                    "{(dialog.message)().unwrap_or_default()}"
                }
                div {
                    class: "flex justify-end gap-2",
                    if let Some(on_confirm) = (dialog.on_confirm)() {
                        button {
                            class: "btn btn-ghost",
                            onclick: move |_| dialog.close(),
                            { t!("no") }
                        }
                        button {
                            class: "btn {dialog.r#type.read().css_btn_class()}",
                            onclick: move |_| {
                                on_confirm.call(());
                                dialog.close();
                            },
                            { t!("yes") }
                        }
                    } else {
                        button {
                            class: "btn {dialog.r#type.read().css_btn_class()}",
                            onclick: move |_| dialog.close(),
                        }
                    }
                }
            }
        }
    }
}
