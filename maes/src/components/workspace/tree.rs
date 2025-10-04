use crate::{components::dialogs::*, prelude::*, services::*};

#[component]
pub fn WorkspaceTree() -> Element {
    let claims = AuthService::claims();
    let mut input_dialog = use_input_dialog();

    let kind = use_context::<Signal<EntityKind>>();
    let mut tree = use_context::<Signal<Vec<TreeNode>>>();
    let mut selected = use_context::<Signal<SelectedItem>>();
    let node_class = if selected.read().id.is_empty() {
        "bg-base-300"
    } else {
        ""
    };

    use_effect(move || {
        api_fetch!(
            GET,
            format!("/api/v1/workspaces/tree/{kind}"),
            on_success = move |body: Vec<TreeNode>| tree.set(body),
        )
    });

    let add_action = {
        let callback = use_callback(move |name: String| {
            api_fetch!(
                POST,
                format!("/api/v1/workspaces/tree/{kind}"),
                CreateWorkspaceTreeNodePayload {
                    name,
                    parent: "".to_string(),
                },
                on_success = move |body: TreeNode| tree.with_mut(|t| t.add_node(body)),
            );
        });
        use_callback(move |_| input_dialog.open(t!("add"), callback, t!("name"), ""))
    };

    let ctx_menu = make_ctx_menu!([(t!("add"), "bi bi-folder-plus", add_action, false, false)]);

    rsx! {
        ul {
            class: "menu flex-wrap",
            li {
                key: "{claims.id}",
                div {
                    class: "semibold text-primary {node_class}",
                    oncontextmenu: move |evt| if claims.is_admin() { ctx_menu(evt) } else { evt.stop_propagation() },
                    onclick: move |_| selected.set(SelectedItem::default()),
                    match kind() {
                        EntityKind::Workspace => rsx! { i { class: "bi bi-person-workspace" } },
                        EntityKind::Quiz => rsx! { i { class: "bi bi-mortarboard" } },
                        EntityKind::Survey => rsx! { i { class: "bi bi-patch-question" } },
                        _ => rsx! {},
                    }
                    "{claims.workspace}"
                }
                ul {
                    if kind() != EntityKind::Workspace || !claims.is_user() {
                        for child in tree.read().root_nodes() {
                            RenderTreeNode {
                                key: "{child}",
                                node_id: "{child}",
                            }
                        }
                    } else {
                        if let Some(node) = tree.iter().find(|n| n.id == claims.node) {
                            RenderTreeNode {
                                key: "{node.id}",
                                node_id: "{node.id}",
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderTreeNode(node_id: ReadOnlySignal<String>) -> Element {
    let claims = AuthService::claims();

    let mut input_dialog = use_input_dialog();

    let kind = use_context::<Signal<EntityKind>>();
    let mut tree = use_context::<Signal<Vec<TreeNode>>>();
    let mut selected = use_context::<Signal<SelectedItem>>();
    let node_class = if *node_id.read() == selected.read().id {
        "bg-base-300"
    } else {
        ""
    };

    let Some(node) = tree.iter().find(|n| n.id == *node_id.read()) else {
        return rsx! {};
    };

    let add_action = {
        let callback = use_callback(move |name: String| {
            api_fetch!(
                POST,
                format!("/api/v1/workspaces/tree/{kind}"),
                CreateWorkspaceTreeNodePayload {
                    name,
                    parent: node_id()
                },
                on_success = move |body: TreeNode| tree.with_mut(|t| t.add_node(body)),
            );
        });
        use_callback(move |_| input_dialog.open(t!("add"), callback, t!("name"), ""))
    };

    let update_action = {
        let node_name = node.name.clone();
        let node_parent = node.parent.clone();
        let callback = use_callback(move |name: String| {
            api_fetch!(
                PATCH,
                format!("/api/v1/workspaces/tree/{kind}/{node_id}"),
                UpdateWorkspaceTreeNodePayload {
                    name,
                    parent: node_parent.clone(),
                },
                on_success = move |body: TreeNode| {
                    tree.with_mut(|t| {
                        if let Some(idx) = t.iter().position(|u| u.id == *node_id.read()) {
                            t[idx].name = body.name;
                            t[idx].parent = body.parent;
                        }
                    });
                },
            );
        });
        use_callback(move |_| input_dialog.open(t!("edit"), callback, t!("name"), &node_name))
    };

    let delete_action = {
        let node_name = node.name.clone();
        let callback = use_callback(move |_| {
            api_fetch!(
                DELETE,
                format!("/api/v1/workspaces/tree/{kind}/{node_id}"),
                on_success = move |body: String| {
                    tree.with_mut(|t| t.remove_node(body));
                    selected.set(selected());
                },
            );
        });
        use_callback(move |_| {
            use_dialog().warning(
                t!("delete-entity-message", name = node_name.clone()),
                Some(callback),
            )
        })
    };

    let (create_entity_title, create_entity_icon, create_entity_action) = match kind() {
        EntityKind::Workspace => (
            "create-user",
            "bi bi-person-plus",
            use_callback(move |_| {
                let callback = use_callback(move |_| selected.set(selected()));
                use_create_user_dialog().open(node_id(), Some(callback));
            }),
        ),
        EntityKind::Quiz => (
            "create-quiz",
            "bi bi-mortarboard",
            use_callback(move |_| {
                let callback = use_callback(move |name: String| {
                    api_fetch!(
                        POST,
                        "/api/v1/manager/quizzes",
                        CreateQuizPayload {
                            name,
                            node: node_id()
                        },
                        on_success = move |body: Quiz| {
                            use_navigator().push(Route::QuizManager { quiz_id: body.id });
                        },
                    )
                });
                input_dialog.open(t!("create-quiz"), callback, t!("name"), "")
            }),
        ),
        EntityKind::Survey => (
            "create-survey",
            "bi bi-patch-question",
            use_callback(move |_| {
                let callback = use_callback(move |name: String| {
                    api_fetch!(
                        POST,
                        "/api/v1/manager/surveys",
                        CreateSurveyPayload {
                            name,
                            node: node_id()
                        },
                        on_success = move |body: Survey| {
                            use_navigator().push(Route::SurveyManager { survey_id: body.id });
                        },
                    )
                });
                input_dialog.open(t!("create-survey"), callback, t!("name"), "")
            }),
        ),
        _ => ("create", "bi bi-plus", use_callback(|_| {})),
    };

    let ctx_menu = make_ctx_menu!([
        (
            t!(create_entity_title),
            create_entity_icon,
            create_entity_action,
            false,
            true
        ),
        (t!("add"), "bi bi-folder-plus", add_action),
        (t!("edit"), "bi bi-pen", update_action),
        (t!("delete"), "bi bi-trash", delete_action),
    ]);

    let select_action = move |_evt: MouseEvent| {
        if let Some(node) = tree().iter().find(|n| n.id == *node_id.read()) {
            selected.set(SelectedItem {
                id: node.id.clone(),
                name: node.name.clone(),
                path: tree.read().node_path(&node.id),
            });
        }
    };

    rsx! {
        li {
            if node.children.is_empty() {
                div {
                    class: "{node_class}",
                    oncontextmenu: move |evt| if claims.is_admin() { ctx_menu(evt) } else { evt.stop_propagation() },
                    onclick: select_action,
                    i { class: "bi bi-folder text-base-content/70" }
                    "{node.name}"
                }
            } else {
                details {
                    summary {
                        class: "{node_class}",
                        oncontextmenu: move |evt| if claims.is_admin() { ctx_menu(evt) } else { evt.stop_propagation() },
                        onclick: select_action,
                        i { class: "bi bi-folder2-open text-base-content/70" }
                        "{node.name}"
                    }
                    ul {
                        for child in node.children.iter() {
                            RenderTreeNode {
                                key: "{child}",
                                node_id: "{child}",
                            }
                        }
                    }
                }
            }
        }
    }
}
