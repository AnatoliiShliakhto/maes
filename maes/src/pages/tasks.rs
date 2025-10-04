use crate::{prelude::*, components::{widgets::*, tasks::*}};

#[component]
pub fn Tasks() -> Element {
    use_context_provider(|| Signal::new(SelectedItem::default()));
    use_context_provider(|| Signal::new(EntityKind::Workspace));

    rsx! {
        SplitPanel {
            left: rsx! {
                div {
                    class: "flex-fixed",
                    TasksList {}
                }                
            },
            right_title: t!("task-manager"),
            right: rsx! {}
        }
    }
}