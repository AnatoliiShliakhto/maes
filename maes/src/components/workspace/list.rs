use crate::{components::dialogs::*, prelude::*, services::*};

#[component]
pub fn WorkspaceList() -> Element {
    let kind = use_context::<Signal<EntityKind>>();
    let selected = use_context::<Signal<SelectedItem>>();
    let mut list = use_context::<Signal<Vec<Entity>>>();

    use_effect(move || {
        api_fetch!(
            GET,
            format!("/api/v1/entities/{kind}/{node}", node = selected.read().id),
            on_success = move |body: Vec<Entity>| list.set(body),
        );
    });

    rsx! {
        ul {
            class: "list w-full",
            for item in list().into_iter() {
                RenderListItemRow { key: "{item.id}", item }
            }
        }
    }
}

#[component]
fn RenderListItemRow(item: ReadSignal<Entity>) -> Element {
    let claims = AuthService::claims();
    let kind = use_context::<Signal<EntityKind>>();
    let mut list = use_context::<Signal<Vec<Entity>>>();
    let item_guard = item.read();
    let mut active = use_context::<Signal<Option<SelectedItem>>>();
    let mut dialog = use_dialog();

    let delete_action = move |evt: MouseEvent| {
        evt.stop_propagation();
        let callback = Callback::new(move |_| {
            api_fetch!(
                DELETE,
                format!("/api/v1/entities/{kind}/{id}", id = item.read().id),
                on_success = move |body: String| list.with_mut(|l| l.retain(|e| e.id != body))
            )
        });
        dialog.warning(
            t!("delete-entity-message", name = item.read().name.clone()),
            Some(callback),
        )
    };

    let click_action = move |evt: MouseEvent| {
        evt.stop_propagation();
        let navigator = use_navigator();
        if active.read().is_some() {
            let item_guard = item.read();
            active.set(Some(SelectedItem {
                id: item_guard.id.clone(),
                name: item_guard.name.clone(),
                path: item_guard.path.clone(),
            }));
            return;
        }

        match kind() {
            EntityKind::Quiz => {
                navigator.push(Route::QuizManager {
                    quiz_id: item.read().id.clone(),
                });
            }
            EntityKind::Survey => {
                navigator.push(Route::SurveyManager {
                    survey_id: item.read().id.clone(),
                });
            }
            _ => (),
        }
    };
    let active_class = if let Some(true) = active.read().as_ref().map(|a| a.id == item_guard.id) {
        "bg-base-300"
    } else {
        ""
    };

    rsx! {
        li {
            class: "list-row hover:bg-base-200 rounded-none group p-0 cursor-pointer {active_class}",
            onclick: click_action,
            div {
                class: "flex items-center justify-center py-2 pl-4 text-xl text-base-content/60",
                match kind() {
                    EntityKind::Quiz => rsx! { i { class: "bi bi-mortarboard" } },
                    EntityKind::Survey => rsx! { i { class: "bi bi-incognito" } },
                    _ => rsx! {}
                }
            }
            div {
                class: "flex flex-col justify-center my-3 gap-1",
                div {
                    class: "font-semibold",
                    "{item_guard.name}"
                }
                div {
                    class: "flex flex-wrap text-xs text-base-content/60 gap-2",
                    span {
                        class: "nowrap whitespace-nowrap inline-flex items-center",
                        i { class: "bi bi-clock text-accent mr-1" }
                        "{item_guard.metadata.updated_at()}"
                    }
                    span {
                        class: "nowrap whitespace-nowrap inline-flex items-center",
                        i { class: "bi bi-person text-primary mr-1" }
                        "{item_guard.metadata.updated_by}"
                    }
                }
            }
            if claims.is_admin() {
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
