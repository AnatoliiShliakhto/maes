use crate::{prelude::*, components::{widgets::*, workspace::*, dialogs::*, students::*}};

#[component]
pub fn Students() -> Element {
    use_init_input_dialog();
    use_init_create_user_dialog();
    use_init_add_student_dialog();
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
//            right_title: t!("students"),
            right: rsx! {
                div {
                    class: "flex-fixed",
                    StudentsList {}
                }
            }
        }
        InputDialogContainer { key: "students-input-dialog" }
        CreateUserDialogContainer { key: "students-create-user-dialog" }
        AddStudentDialogContainer { key: "students-add-student-dialog" }
        ContextMenuContainer { key: "students-ctx-menu" }
    }
}