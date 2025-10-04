use crate::{
    components::{dialogs::*, inputs::*},
    prelude::*,
    services::*,
};

#[component]
pub fn Login() -> Element {
    let config = ConfigService::read();
    let mut workspaces = use_signal(Vec::<WorkspaceMetadata>::new);
    let mut create_workspace_dialog = use_init_create_workspace_dialog();

    use_effect(move || {
        if (create_workspace_dialog.is_visible)() {
            return;
        };

        api_fetch!(
            GET,
            "/api/v1/workspaces",
            on_success = move |body: Vec<WorkspaceMetadata>| workspaces.set(body)
        )
    });

    let login_action = move |evt: FormEvent| {
        evt.stop();
        let (Some(workspace), Some(login), Some(password)) =
            form_values!(evt, "workspace", "login", "password")
        else {
            ToastService::error(t!("missing-fields"));
            return;
        };
        ConfigService::with_mut(|config| {
            config.recent.workspace = workspace.clone();
            config.recent.login = login.clone();
        }).ok();
        AuthService::login(workspace, login, password);
    };

    rsx! {
        div {
            class: "grid grow place-items-center overflow-y-auto",
            div {
                class: "hero",
                div {
                    class: "hero-content flex-col lg:flex-row-reverse w-full",
                    div {
                        class: "text-center lg:text-left w-full max-w-sm items-center justify-center",
                        h1 {
                            class: "text-3xl font-bold text-center",
                            { t!("login-form-title") }
                        }
                        p {
                            class: "py-6 px-5",
                            { t!("login-form-announcement") }
                        }
                    }
                    div {
                        class: "card card-border bg-base-100 w-sm shadow-lg",
                        div {
                            class: "card-body pt-4",
                            form {
                                id: "login-form",
                                autocomplete: "off",
                                onsubmit: login_action,
                                input {
                                    r#type: "submit",
                                    style: "position: absolute; left: -9999px; width: 1px; height: 1px;",
                                    tabindex: -1,
                                }
                                Select {
                                    name: "workspace",
                                    required: true,
                                    label: rsx! { span { i { class: "bi bi-person-workspace mr-1" } { t!("workspace") } } },
                                    for workspace in workspaces.read().iter() {
                                        option {
                                            key: "{workspace.id}",
                                            selected: workspace.id.eq(&config.recent.workspace),
                                            value: "{workspace.id}",
                                            "{workspace.name}"
                                        }
                                    }
                                }
                                div {
                                    class: "flex w-full justify-end gap-1 mt-2 text-sm",
                                    a {
                                        class: "link link-hover text-accent",
                                        onclick: move |_| create_workspace_dialog.open(),
                                        { t!("create-new-workspace") }
                                    }
                                    span { { t!("or") } }
                                    a {
                                        class: "link link-hover text-accent",
                                        //onclick: import, todo
                                        { t!("import-existed-workspace") }
                                    }
                                }
                                TextInputComponent {
                                    class: "mt-5",
                                    label: rsx! { span { i { class: "bi bi-person mr-1" } { t!("login") } } },
                                    name: "login",
                                    placeholder: t!("login"),
                                    minlength: 4,
                                    maxlength: 20,
                                    required: true,
                                    initial_value: "{config.recent.login}"
                                }
                                TextInputComponent {
                                    class: "mt-5",
                                    r#type: "password",
                                    label: rsx! { span { i { class: "bi bi-key mr-1" } { t!("password") } } },
                                    name: "password",
                                    placeholder: t!("password"),
                                    maxlength: 30,
                                }
                            }
                            button {
                                class: "btn btn-primary mt-5",
                                form: "login-form",
                                { t!("signin") }
                            }
                        }
                    }
                }
            }
        }
        CreateWorkspaceDialogContainer { key: "create-ws-dialog-container" }
    }
}
