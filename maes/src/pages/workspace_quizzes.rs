use crate::{
    components::{dialogs::*, widgets::*, workspace::*},
    prelude::*,
    services::*,
};

#[component]
pub fn WorkspaceQuizzes() -> Element {
    if !AuthService::claims().is_supervisor() {
        return rsx! {};
    }
    use_init_input_dialog();
    use_init_context_menu();

    use_context_provider(|| Signal::new(EntityKind::Quiz));
    use_context_provider(|| Signal::new(SelectedItem::default()));
    use_context_provider(|| Signal::new(None::<SelectedItem>));
    use_context_provider(|| Signal::new(Vec::<TreeNode>::new()));
    use_context_provider(|| Signal::new(Vec::<Entity>::new()));

    rsx! {
        SplitPanel {
            left_title: t!("quizzes-navigator"),
            left: rsx! {
                div {
                    class: "flex-scrollable",
                    WorkspaceTree {}
                }
            },
            right_title: t!("quizzes"),
            right: rsx! {
                div {
                    class: "flex-scrollable",
                    WorkspaceList {}
                }
            }
        }
        InputDialogContainer { key: "ws-quiz-input-dialog" }
        ContextMenuContainer { key: "ws-quiz-ctx-menu" }
    }
}
