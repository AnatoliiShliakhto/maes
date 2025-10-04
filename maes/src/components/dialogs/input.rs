use crate::prelude::*;

#[derive(Default, Copy, Clone)]
pub struct InputDialog {
    pub is_visible: Signal<bool>,
    title: Signal<Option<String>>,
    on_confirm: Signal<Option<Callback<String>>>,
    placeholder: Signal<Option<String>>,
    init_value: Signal<Option<String>>,
}

impl InputDialog {
    pub fn open(
        &mut self,
        title: impl Into<String>,
        on_confirm: Callback<String>,
        placeholder: impl Into<String>,
        init_value: impl Into<String>,
    ) {
        self.title.set(Some(title.into()));
        self.on_confirm.set(Some(on_confirm));
        self.placeholder.set(Some(placeholder.into()));
        self.init_value.set(Some(init_value.into()));
        self.is_visible.set(true);
    }

    pub fn close(&mut self) {
        self.title.set(None);
        self.on_confirm.set(None);
        self.placeholder.set(None);
        self.init_value.set(None);
        self.is_visible.set(false);
    }
}

pub fn use_init_input_dialog() -> InputDialog {
    use_context_provider(InputDialog::default)
}

pub fn use_input_dialog() -> InputDialog {
    use_context()
}

#[component]
pub fn InputDialogContainer() -> Element {
    let mut dialog = use_input_dialog();
    if !(dialog.is_visible)() {
        return rsx! {};
    }
    let mut value = use_signal(String::new);

    let submit_action = use_callback(move |_| {
        let value = value().trim().to_string();
        if value.is_empty() {
            return;
        }
        if let Some(on_confirm) = (dialog.on_confirm)() {
            on_confirm.call(value)
        }
        dialog.close()
    });

    rsx! {
        dialog {
            class: "modal modal-open",
            div {
                class: "modal-box flex flex-col gap-5",
                onclick: |evt| evt.stop_propagation(),
                h3 {
                    class: "text-lg semibold text-accent",
                    "{(dialog.title)().unwrap_or_default()}"
                }
                input {
                    class: "input input-bordered w-full",
                    r#type: "text",
                    placeholder: "{(dialog.placeholder)().unwrap_or_default()}",
                    autofocus: true,
                    initial_value: "{(dialog.init_value)().unwrap_or_default()}",
                    oninput: move |evt| value.set(evt.value()),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Enter {
                            submit_action(())
                        } else if evt.key() == Key::Escape {
                            dialog.close()
                        }
                    }
                }
                div {
                    class: "flex justify-end gap-2",
                    button {
                        class: "btn btn-ghost",
                        onclick: move |_| dialog.close(),
                        { t!("no") }
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| submit_action.call(()),
                        { t!("yes") }
                    }
                }
            }
        }
    }
}
