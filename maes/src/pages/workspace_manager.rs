use crate::{
    components::{dialogs::*, widgets::*, workspace::*},
    prelude::*,
    services::*,
};

#[component]
pub fn WorkspaceManager() -> Element {
    if !AuthService::claims().is_supervisor() {
        return rsx! {};
    }
    use_init_dialog();
    use_init_input_dialog();
    use_init_create_user_dialog();
    use_init_context_menu();

    use_context_provider(|| Signal::new(EntityKind::Workspace));
    use_context_provider(|| Signal::new(SelectedItem::default()));
    use_context_provider(|| Signal::new(Vec::<TreeNode>::new()));
    use_context_provider(|| Signal::new(Vec::<Entity>::new()));

    rsx! {
        SplitPanel {
            left_title: t!("unit-navigator"),
            left: rsx! {
                div {
                    class: "flex-scrollable",
                    WorkspaceTree {}
                }
            },
            right_title: t!("users"),
            right: rsx! {
                div {
                    class: "flex-scrollable",
                    WorkspaceUsers {}
                }
            }
        }
        DialogContainer { key: "ws-dialog" }
        InputDialogContainer { key: "ws-input-dialog" }
        CreateUserDialogContainer { key: "ws-create-user-dialog" }
        ContextMenuContainer { key: "ws-ctx-menu" }
    }
}
