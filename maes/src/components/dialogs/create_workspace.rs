use crate::{components::inputs::*, prelude::*, services::*};

#[derive(Default, Copy, Clone)]
pub struct CreateWorkspaceDialog {
    pub is_visible: Signal<bool>,
}

impl CreateWorkspaceDialog {
    pub fn open(&mut self) {
        self.is_visible.set(true);
    }

    pub fn close(&mut self) {
        self.is_visible.set(false);
    }
}

pub fn use_init_create_workspace_dialog() -> CreateWorkspaceDialog {
    use_context_provider(CreateWorkspaceDialog::default)
}

pub fn use_create_workspace_dialog() -> CreateWorkspaceDialog {
    use_context()
}

#[component]
pub fn CreateWorkspaceDialogContainer() -> Element {
    let mut is_visible = use_create_workspace_dialog().is_visible;
    if !is_visible() {
        return rsx! {};
    }

    let create_action = move |evt: FormEvent| {
        evt.stop();
        let (Some(name), Some(username), Some(login), Some(password)) =
            form_values!(evt, "name", "username", "login", "password")
        else {
            ToastService::error(t!("missing-fields"));
            return;
        };
        api_call!(
            POST,
            "/api/v1/workspaces",
            CreateWorkspacePayload {
                name,
                username,
                login,
                password,
            },
            on_success = move || is_visible.set(false)
        );
    };

    if !is_visible() {
        return rsx! {};
    };
    
    rsx! {
        dialog {
            class: "modal modal-open",
            div {
                class: "modal-box flex flex-col gap-5",
                onclick: |evt| evt.stop_propagation(),
                h3 {
                    class: "text-lg font-semibold",
                    { t!("create-workspace") }
                }

                form {
                    id: "create-workspace-form",
                    autocomplete: "off",
                    onsubmit: create_action,

                    input {
                        r#type: "submit",
                        style: "position: absolute; left: -9999px; width: 1px; height: 1px;",
                        tabindex: -1,
                    }

                    TextInputComponent {
                        label: rsx! { span { i { class: "bi bi-person-workspace mr-1" } { t!("workspace-name") } } },
                        name: "name",
                        placeholder: t!("workspace-name"),
                        minlength: 5,
                        maxlength: 100,
                        required: true,
                    }

                    fieldset {
                        class: "fieldset flex flex-col p-4 border border-base-300 rounded-(--radius-box) mt-3 gap-5",
                        legend {
                            class: "fieldset-legend",
                            i { class: "bi bi-person-vcard mr-1" }
                            { t!("workspace-administrator") }
                        }
                        TextInputComponent {
                            label: rsx! { span { i { class: "bi bi-person-circle mr-1" } { t!("username") } } },
                            name: "username",
                            placeholder: t!("username"),
                            minlength: 5,
                            maxlength: 100,
                            required: true,
                        }
                        TextInputComponent {
                            label: rsx! { span { i { class: "bi bi-person mr-1" } { t!("login") } } },
                            name: "login",
                            placeholder: t!("login"),
                            minlength: 4,
                            maxlength: 20,
                            required: true,
                        }
                        TextInputComponent {
                            r#type: "password",
                            label: rsx! { span { i { class: "bi bi-key mr-1" } { t!("password") } } },
                            name: "password",
                            placeholder: t!("password"),
                            maxlength: 30,
                        }
                    }

                    div {
                        class: "flex justify-end gap-2 mt-7",
                        button {
                            class: "btn btn-ghost",
                            onclick: move |evt| {
                                evt.stop_propagation();
                                evt.prevent_default();
                                is_visible.set(false)
                            },
                            { t!("no") }
                        }
                        button {
                            class: "btn btn-primary",
                            form: "create-workspace-form",
                            { t!("yes") }
                        }
                    }
                }
            }
        }
    }
}
