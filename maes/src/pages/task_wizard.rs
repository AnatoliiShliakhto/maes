use crate::{
    components::{dialogs::*, widgets::*, wizards::task::*},
    prelude::*,
};

#[component]
pub fn TaskWizard() -> Element {
    use_init_input_dialog();
    use_init_create_user_dialog();
    use_init_add_student_dialog();
    use_init_context_menu();
    let steps = use_init_steps(vec![t!("start"), t!("task"), t!("unit"), t!("finish")]);

    use_context_provider(|| Signal::new(EntityKind::Quiz));
    use_context_provider(|| Signal::new((SelectedItem::default(), SelectedItem::default())));

    rsx! {
        div {
            class: "flex-fixed mx-5 mb-5",
            div {
                class: "flex shrink-0",
                StepsContainer {}
            }

            match steps.current()() {
                1 => rsx! { TaskWizardStep1 {} },
                2 => rsx! { TaskWizardStep2 {} },
                3 => rsx! { TaskWizardStep3 {} },
                4 => rsx! { TaskWizardStep4 {} },
                _ => rsx! {},
            }
        }

        InputDialogContainer { key: "wizard-quiz-input-dialog" }
        CreateUserDialogContainer { key: "wizard-create-user-dialog" }
        AddStudentDialogContainer { key: "wizard-add-student-dialog" }
        ContextMenuContainer { key: "wizard-quiz-ctx-menu" }
    }
}
