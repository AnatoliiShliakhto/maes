use crate::prelude::*;

#[derive(Default, Copy, Clone, PartialEq)]
struct InputDialogArgs {
    pub is_visible: Signal<bool>,
    pub title: Signal<String>,
    pub placeholder: Signal<String>,
    pub on_submit: Signal<Callback<String>>,
    pub value: Signal<String>,
}

impl InputDialogArgs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hide(&mut self) {
        self.is_visible.set(false)
    }

    pub fn is_visible(&self) -> bool {
        (self.is_visible)()
    }

    pub fn value(&self) -> String {
        (self.value)()
    }

    pub fn on_submit(&self) -> Callback<String> {
        (self.on_submit)()
    }

    pub fn show(
        &mut self,
        title: impl Into<String>,
        placeholder: impl Into<String>,
        value: impl Into<String>,
        on_submit: Callback<String>,
    ) {
        self.title.set(title.into());
        self.placeholder.set(placeholder.into());
        self.on_submit.set(on_submit);
        self.value.set(value.into());
        self.is_visible.set(true)
    }
}

static INPUT_DIALOG: GlobalSignal<InputDialogArgs> = GlobalSignal::new(InputDialogArgs::new);

#[derive(Copy, Clone)]
pub struct InputDialog;

impl InputDialog {
    pub fn show(
        title: impl Into<String>,
        value: impl Into<String>,
        placeholder: impl Into<String>,
        on_submit: Callback<String>,
    ) {
        INPUT_DIALOG
            .signal()
            .with_mut(|dialog| dialog.show(title, placeholder, value, on_submit))
    }
}

#[component]
pub fn InputDialogContainer() -> Element {
    let input_dialog = INPUT_DIALOG.signal();
    if !input_dialog().is_visible() {
        return rsx! {};
    }

    rsx! {
        dialog {
            class: "modal modal-open",
            div {
                class: "modal-box flex flex-col gap-5",
                onclick: |evt| evt.stop_propagation(),
                h3 {
                    class: "text-lg semibold text-accent",
                    "{input_dialog().title}"
                }
                input {
                    class: "input input-bordered w-full",
                    r#type: "text",
                    placeholder: "{input_dialog().placeholder}",
                    autofocus: true,
                    initial_value: "{input_dialog().value}",
                    oninput: move |evt| input_dialog().value.set(evt.value()),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Enter {
                            let value = input_dialog().value().trim().to_string();
                            if !value.is_empty() {
                                input_dialog().on_submit().call(value);
                            }
                            input_dialog().hide();
                        } else if evt.key() == Key::Escape {
                            input_dialog().hide();
                        }
                    }
                }
                div {
                    class: "flex justify-end gap-2",
                    button {
                        class: "btn btn-ghost",
                        onclick: move |_| input_dialog().hide(),
                        { t!("no") }
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            let value = input_dialog().value().trim().to_string();
                            if !value.is_empty() {
                                input_dialog().on_submit().call(value);
                                input_dialog().hide()
                            }
                        },
                        { t!("yes") }
                    }
                }
            }
        }
    }
}
