use crate::{components::dialogs::*, prelude::*, services::*};

#[component]
pub fn WorkspaceUsers() -> Element {
    let mut users = use_context_provider(|| Signal::new(Vec::<WorkspaceUser>::new()));
    let selected = use_context::<Signal<SelectedItem>>();

    use_effect(move || {
        api_fetch!(
            GET,
            format!("/api/v1/workspaces/users/{node}", node = selected.read().id),
            on_success = move |body: Vec<WorkspaceUser>| users.set(body),
        );
    });

    rsx! {
        ul {
            class: "list w-full",
            for user in users().into_iter() {
                RenderUserRow { key: "{user.id}", user }
            }
        }
    }
}

#[component]
fn RenderUserRow(user: ReadSignal<WorkspaceUser>) -> Element {
    let claims =  AuthService::claims();

    let mut users = use_context::<Signal<Vec<WorkspaceUser>>>();
    let user_guard = user.read();
    let first_chars = extract_first_chars(&user_guard.username);

    let delete_action = move |_| {
        let callback = use_callback(move |_| {
            api_fetch!(
                DELETE,
                format!("/api/v1/workspaces/users/{id}", id = user.read().id),
                on_success = move |body: String| users.with_mut(|u| u.retain(|u| u.id != body)),
            )
        });
        use_dialog().warning(
            t!(
                "delete-user-message",
                username = user.read().username.clone()
            ),
            Some(callback),
        )
    };

    rsx! {
        li {
            class: "list-row hover:bg-base-200 rounded-none p-0 group",
            div {
                class: "avatar avatar-placeholder justify-center size-10 my-3 ml-3",
                div {
                    class: format!("flex w-10 rounded-full items-center justify-center {class}", class = match user_guard.role {
                        WorkspaceRole::Admin => "bg-error/60 text-error-content",
                        WorkspaceRole::Supervisor => "bg-success/60 text-success-content",
                        _ => "bg-neutral/60 text-neutral-content",
                    }),
                    span {
                        class: "text",
                        "{first_chars}"
                    }
                }
            }
            div {
                class: "flex flex-col justify-center my-3 gap-1",
                div {
                    class: "font-semibold",
                    "{user_guard.username}"
                }
                div {
                    class: "text-xs text-base-content/60",
                    if user_guard.path.is_empty() {
                        i { class: "bi bi-asterisk text-[.5rem] text-error/60" }
                    } else  {
                        "{user_guard.path}"
                    }
                }
            }
            if claims.is_admin() && claims.id != user_guard.id {
                div {
                    class: "hidden group-hover:flex h-full w-14 items-center justify-center",
                    class: "text-base-content/60 hover:text-error-content hover:bg-error cursor-pointer",
                    onclick: delete_action,
                    i { class: "bi bi-trash text-lg" }
                }
            }
        }
    }
}
