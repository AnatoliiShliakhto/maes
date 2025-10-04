use crate::{components::inputs::*, prelude::*, services::*};

#[derive(Default, Copy, Clone)]
pub struct CreateUserDialog {
    pub is_visible: Signal<bool>,
    node: Signal<Option<String>>,
    on_confirm: Signal<Option<Callback>>,
}

impl CreateUserDialog {
    pub fn open(&mut self, node: String, on_confirm: Option<Callback>) {
        self.node.set(Some(node));
        self.on_confirm.set(on_confirm);
        self.is_visible.set(true);
    }

    pub fn close(&mut self) {
        self.node.set(None);
        self.on_confirm.set(None);
        self.is_visible.set(false);
    }
}

pub fn use_init_create_user_dialog() -> CreateUserDialog {
    use_context_provider(CreateUserDialog::default)
}

pub fn use_create_user_dialog() -> CreateUserDialog {
    use_context()
}

#[component]
pub fn CreateUserDialogContainer() -> Element {
    let mut dialog = use_create_user_dialog();

    if !(dialog.is_visible)() {
        return rsx! {};
    };

    let create_action = move |evt: FormEvent| {
        evt.stop();
        let (Some(role), Some(username), Some(login), Some(password)) =
            form_values!(evt, "role", "username", "login", "password")
        else {
            ToastService::error(t!("missing-fields"));
            return;
        };

        let role = WorkspaceRole::from(role);

        api_call!(
            POST,
            "/api/v1/workspaces/users",
            CreateWorkspaceUserPayload {
                username,
                login,
                password,
                node: (dialog.node)().unwrap_or_default(),
                role,
            },
            on_success = move || {
                if let Some(on_confirm) = (dialog.on_confirm)() {
                    on_confirm.call(());
                }
                dialog.close();
            }
        )
    };

    rsx! {
        dialog {
            class: "modal modal-open",
            div {
                class: "modal-box flex flex-col gap-5",
                onclick: |evt| evt.stop_propagation(),
                h3 {
                    class: "text-lg font-semibold text-accent",
                    { t!("create-user") }
                }

                form {
                    id: "create-user-form",
                    autocomplete: "off",
                    onsubmit: create_action,
                    input {
                        r#type: "submit",
                        style: "position: absolute; left: -9999px; width: 1px; height: 1px;",
                        tabindex: -1,
                    }
                    div {
                        key: "user-role-filter",
                        class: "filter",
                        input {
                            class: "btn btn-square filter-reset",
                            r#type: "radio",
                            name: "role",
                            value: "user",
                            // r#type: "reset",
                            aria_label: "x",
                            initial_checked: true
                        }
                        input {
                            class: "btn",
                            r#type: "radio",
                            name: "role",
                            value: "admin",
                            aria_label: t!("administrator")
                        }
                        input {
                            class: "btn",
                            r#type: "radio",
                            name: "role",
                            value: "supervisor",
                            aria_label: t!("supervisor")
                        }
                        // input {
                        //     class: "btn",
                        //     r#type: "radio",
                        //     name: "role",
                        //     value: "user",
                        //     aria_label: t!("user")
                        // }
                    }
                    fieldset {
                        class: "fieldset flex flex-col p-4 border border-base-300 rounded-(--radius-box) mt-3 gap-5",
                        legend {
                            class: "fieldset-legend",
                            i { class: "bi bi-person-vcard mr-1" }
                            { t!("credentials") }
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
                                dialog.close();
                            },
                            { t!("no") }
                        }
                        button {
                            form: "create-user-form",
                            class: "btn btn-primary",
                            { t!("yes") }
                        }
                    }
                }
            }
        }
    }
}
