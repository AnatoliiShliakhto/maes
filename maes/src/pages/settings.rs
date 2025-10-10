use crate::{
    components::{dialogs::*, widgets::*},
    prelude::*,
    services::*,
};

#[component]
pub fn Settings() -> Element {
    let config = ConfigService::read();
    use_init_dialog();

    let save_settings_action = move |evt: FormEvent| {
        evt.stop();
        let (Some(host), Some(ident), Some(ssid), Some(password), direct) =
            form_values!(evt, "host", "ident", "ssid", "password", "wifi-direct")
        else {
            ToastService::error(t!("missing-fields"));
            return;
        };
        let Ok((scheme, host, port)) = parse_scheme_host_port(&host) else {
            ToastService::error(t!("host-format-error"));
            return;
        };
        if let Err(e) = ConfigService::with_mut(|config| {
            config.server.host = format!("{scheme}://{host}:{port}");
            config.server.ident = ident;
            config.wifi.ssid = ssid;
            config.wifi.password = password;
            config.wifi.direct = direct.is_some();
        }) {
            ToastService::error(t!(e.to_string()))
        } else {
            ToastService::success(t!("config-saved"))
        }
    };

    rsx! {
        Panel {
            title: t!("settings"),
            div {
                class: "flex-scrollable ml-5",
                form {
                    id: "settings-form",
                    class: "flex flex-col gap-5 mt-5",
                    autocomplete: "off",
                    onsubmit: save_settings_action,
                    input {
                        r#type: "submit",
                        style: "position: absolute; left: -9999px; width: 1px; height: 1px;",
                        tabindex: -1,
                    }
                    div {
                        class: "card",
                        div {
                            class: "card-title text-2xl text-primary",
                            i { class: "bi bi-pc-horizontal mr-2" }
                            { t!("server-settings") }
                            div {
                                class: "flex w-full items-center justify-end pr-10 gap-5",
                                div {
                                    class: "tooltip tooltip-bottom",
                                    "data-tip": t!("reboot-tooltip"),
                                    i { class: "bi bi-info-circle text-info text-xl pl-2" }
                                }
                                button {
                                    r#type: "submit",
                                    form: "settings-form",
                                    class: "btn btn-accent",
                                    i { class: "bi bi-floppy mr-2" }
                                    { t!("save-settings")}
                                }
                            }
                        }
                        div {
                            class: "card-body",
                            div {
                                class: "flex flex-1 gap-2 items-center",
                                label {
                                    class: "input validator",
                                    span { class: "label", i { class: "bi bi-pc-display" } }
                                    input {
                                        r#type: "text",
                                        name: "host",
                                        required: true,
                                        minlength: 5,
                                        maxlength: 100,
                                        placeholder: t!("host"),
                                        initial_value: "{config.server.host}",
                                    }
                                }
                            }
                            div {
                                class: "flex flex-1 gap-2 items-center",
                                label {
                                    class: "input validator",
                                    span { class: "label", i { class: "bi bi-key" } }
                                    input {
                                        r#type: "text",
                                        name: "ident",
                                        required: true,
                                        minlength: 6,
                                        maxlength: 20,
                                        placeholder: t!("ident"),
                                        initial_value: "{config.server.ident}",
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "card",
                        div {
                            class: "card-title text-2xl text-primary",
                            i { class: "bi bi-router mr-2" }
                            { t!("wifi-settings") }
                        }
                        div {
                            class: "card-body",
                            label {
                                input {
                                    class: "toggle checked:toggle-accent",
                                    name: "wifi-direct",
                                    r#type: "checkbox",
                                    initial_checked: config.wifi.direct
                                }
                                span {
                                    class: "ml-2 text-base-content/70",
                                    { t!("wifi-direct") }
                                }
                            }
                            label {
                                class: "input validator",
                                span { class: "label", i { class: "bi bi-wifi" } }
                                input {
                                    r#type: "text",
                                    name: "ssid",
                                    required: true,
                                    minlength: 4,
                                    maxlength: 30,
                                    placeholder: t!("wifi-ssid"),
                                    initial_value: "{config.wifi.ssid}",
                                }
                            }
                            label {
                                class: "input validator",
                                span { class: "label", i { class: "bi bi-key" } }
                                input {
                                    r#type: "text",
                                    name: "password",
                                    minlength: 8,
                                    maxlength: 15,
                                    placeholder: t!("wifi-password"),
                                    initial_value: "{config.wifi.password}",
                                }
                            }
                        }
                    }
                }
                div {
                    class: "card pt-5 pr-5",
                    div {
                        class: "card-title text-2xl text-primary",
                        i { class: "bi bi-person-workspace mr-2" }
                        { t!("workspaces") }
                    }
                    div {
                        class: "card-body",
                        RenderWorkspaces {}
                    }
                }
            }
        }
        DialogContainer { key: "settings-dialog-container" }
    }
}

#[component]
fn RenderWorkspaces() -> Element {
    let mut workspaces = use_signal(Vec::<WorkspaceMetadata>::new);
    let mut dialog = use_dialog();

    use_effect(move || {
        api_fetch!(
            GET,
            "/api/v1/workspaces",
            on_success = move |body: Vec<WorkspaceMetadata>| workspaces.set(body),
        )
    });

    let delete_action = use_callback(move |ws: WorkspaceMetadata| {
        let id = ws.id.clone();
        let name = ws.name.clone();
        let callback = use_callback(move |_| {
            to_owned![id, name];
            api_fetch!(
                DELETE,
                format!("/api/v1/workspaces/{id}"),
                on_success = move |body: String| {
                    if AuthService::claims().workspace == name
                    {
                        AuthService::logout();
                    } else {
                        workspaces.with_mut(|ws| ws.retain(|ws| ws.id != body));
                    }
                }
            )
        });

        dialog.warning(
            t!("delete-workspace-message", name = ws.name),
            Some(callback),
        )
    });

    rsx! {
        ul {
            class: "list w-full",
            for ws in workspaces.iter() {
                li {
                    class: "list-row hover:bg-base-200 group p-0 overflow-hidden",
                    div {
                        class: "flex flex-col justify-center m-3 list-col-grow gap-1",
                        div {
                            class: "font-semibold px-1",
                            i { class: "bi bi-file-lock mr-2" }
                            "{ws.name}"
                        }
                    }
                    div {
                        class: "hidden group-hover:flex h-full w-14 items-center justify-center",
                        class: "text-base-content/60 hover:text-error-content hover:bg-error cursor-pointer",
                        onclick: {
                            let ws = ws.clone();
                            move |_| delete_action(ws.clone())
                        },
                        i { class: "bi bi-trash text-lg" }
                    }
                }
            }
        }
    }
}
