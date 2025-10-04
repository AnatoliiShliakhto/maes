use crate::{components::inputs::*, prelude::*, services::*};

#[derive(Default, Copy, Clone)]
pub struct AddStudentDialog {
    pub is_visible: Signal<bool>,
    node: Signal<Option<String>>,
    on_confirm: Signal<Option<Callback<Vec<Student>>>>,
}

impl AddStudentDialog {
    pub fn open(&mut self, node: String, on_confirm: Option<Callback<Vec<Student>>>) {
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

pub fn use_init_add_student_dialog() -> AddStudentDialog {
    use_context_provider(AddStudentDialog::default)
}

pub fn use_add_student_dialog() -> AddStudentDialog {
    use_context()
}

#[component]
pub fn AddStudentDialogContainer() -> Element {
    let mut dialog = use_add_student_dialog();

    if !(dialog.is_visible)() {
        return rsx! {};
    };

    let add_action = move |evt: FormEvent| {
        evt.stop();
        let (rank, Some(name),) =
            form_values!(evt, "rank", "name")
        else {
            ToastService::error(t!("missing-fields"));
            return;
        };

        api_fetch!(
            POST,
            format!("/api/v1/students/{}", (dialog.node)().unwrap_or_default()),
            vec![AddStudentPayload {
                rank,
                name,
            }],
            on_success = move |body: Vec<Student>| {
                if let Some(on_confirm) = (dialog.on_confirm)() {
                    on_confirm.call(body);
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
                    { t!("add-student") }
                }

                form {
                    id: "add-student-form",
                    autocomplete: "off",
                    onsubmit: add_action,
                    input {
                        r#type: "submit",
                        style: "position: absolute; left: -9999px; width: 1px; height: 1px;",
                        tabindex: -1,
                    }

                    fieldset {
                        class: "fieldset flex flex-col p-4 border border-base-300 rounded-(--radius-box) mt-3 gap-5",
                        legend {
                            class: "fieldset-legend",
                            i { class: "bi bi-person-vcard mr-1" }
                            { t!("student") }
                        }
                        TextInputComponent {
                            label: rsx! { span { i { class: "bi bi-star mr-1" } { t!("rank") } } },
                            name: "rank",
                            placeholder: t!("rank"),
                            minlength: 0,
                            maxlength: 30,
                        }
                        TextInputComponent {
                            label: rsx! { span { i { class: "bi bi-person-circle mr-1" } { t!("username") } } },
                            name: "name",
                            placeholder: t!("username"),
                            minlength: 5,
                            maxlength: 150,
                            required: true,
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
                            form: "add-student-form",
                            class: "btn btn-primary",
                            { t!("yes") }
                        }
                    }
                }
            }
        }
    }
}
