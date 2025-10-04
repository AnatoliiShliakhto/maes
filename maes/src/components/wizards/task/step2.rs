use crate::{
    components::{widgets::*, workspace::*},
    prelude::*,
};

#[component]
pub fn TaskWizardStep2() -> Element {
    let mut step = use_steps().current();
    let kind = use_context::<Signal<EntityKind>>();
    let mut task = use_context::<Signal<(SelectedItem, SelectedItem)>>();

    use_context_provider(|| Signal::new(SelectedItem::default()));
    use_context_provider(|| Signal::new(Vec::<TreeNode>::new()));
    use_context_provider(|| Signal::new(Vec::<Entity>::new()));
    let selected = use_context_provider(|| Signal::new(Some(task.read().0.clone())));

    rsx! {
        div {
            class: "card flex-fixed bg-base-100 shadow",
            div {
                class: "card-body flex-fixed items-center gap-5",
                h2 {
                    class: "text-2xl font-bold",
                    { t!("task-wizard-step-2-title", kind = kind.read().as_str()) }
                }

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
                            class: "flex-scrollable",
                            WorkspaceList {}
                        }
                    }
                }
            }
            div {
                class: "card-actions justify-between mx-10 mb-5",
                button {
                    class: "btn",
                    onclick: move |_| {
                        if let Some(item) = &*selected.read() {
                            task.with_mut(|t| t.0 = item.clone());
                        }
                        step.set(step() - 1);
                    },
                    { t!("previous") }
                }
                button {
                    class: format!("btn btn-primary {class}", class =
                        if selected.read().as_ref().map(|s| s.id.is_empty()).unwrap_or_default() { "btn-disabled" } else { "" }
                    ),
                    onclick: move |_| {
                        if let Some(item) = selected.read().as_ref() {
                            task.with_mut(|t| t.0 = item.clone());
                            step.set(step() + 1)
                        }
                    },
                    { t!("next") }
                }
            }
        }
    }
}
