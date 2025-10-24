use crate::{
    components::{dialogs::*, widgets::*, workspace::*},
    prelude::*,
    services::*,
};

#[component]
pub fn WorkspaceQuizzes() -> Element {
    let claims = AuthService::claims();
    if !claims.is_supervisor() {
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
            // left_title: t!("quizzes-navigator"),
            left: rsx! {
                div {
                    class: "flex flex-nowrap shrink-0 w-full gap-2 px-3 pt-2 items-center h-10",
                    i { class: "bi bi-three-dots-vertical" }
                    div {
                        class: "w-full",
                        { t!("quizzes-navigator") }
                    }
                    if claims.is_admin() {
                        ul {
                            class: "menu menu-horizontal p-0 m-0 text-base-content",
                            li {
                                button {
                                    class: "hover:text-success",
                                    onclick: move |_| Exchange::export(vec![]),
                                    i { class: "bi bi-floppy" }
                                    { t!("export") }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "h-0.25 bg-base-300 mx-4 my-1",
                }
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
