use crate::{
    components::{widgets::*, workspace::*, students::*},
    prelude::*,
};

#[component]
pub fn TaskWizardStep3() -> Element {
    let mut step = use_steps().current();
    let kind = use_context::<Signal<EntityKind>>();
    let mut task = use_context::<Signal<(SelectedItem, SelectedItem)>>();

    use_context_provider(|| Signal::new(EntityKind::Workspace));
    use_context_provider(|| Signal::new(None::<SelectedItem>));
    use_context_provider(|| Signal::new(Vec::<TreeNode>::new()));
    use_context_provider(|| Signal::new(Vec::<Entity>::new()));
    let selected = use_context_provider(|| Signal::new(task.read().1.clone()));

    rsx! {
        div {
            class: "card flex-fixed bg-base-100 shadow",
            div {
                class: "card-body flex-fixed items-center gap-5",
                h2 {
                    class: "text-2xl font-bold",
                    { t!("task-wizard-step-3-title") }
                }

                if *kind.read() == EntityKind::Quiz {
                    SplitPanel {
                        //left_title: t!("quizzes-navigator"),
                        left_class: "shadow-none",
                        left: rsx! {
                            div {
                                class: "flex-scrollable",
                                WorkspaceTree {}
                            }
                        },
                        // right_title: t!("quizzes"),
                        right_class: "shadow-none border-1 border-base-200",
                        right: rsx! {
                            div {
                                class: "flex-fixed",
                                StudentsList {}
                            }
                        }
                    }
                } else {
                    Panel {
                        class: "shadow-none",
                        div {
                            class: "flex-scrollable",
                            WorkspaceTree {}
                        }
                    }
                }
            }
            div {
                class: "card-actions justify-between mx-10 mb-5",
                button {
                    class: "btn",
                    onclick: move |_| {
                        task.with_mut(|t| t.1 = selected());
                        step.set(step() - 1);
                    },
                    { t!("previous") }
                }
                button {
                    class: format!("btn btn-primary {class}", class =
                        if selected.read().id.is_empty() { "btn-disabled" } else { "" }
                    ),
                    onclick: move |_| {
                        task.with_mut(|t| t.1 = selected());
                        step.set(step() + 1)
                    },
                    { t!("next") }
                }
            }
        }
    }
}
