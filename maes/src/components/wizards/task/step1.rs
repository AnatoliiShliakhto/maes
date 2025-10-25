use crate::{components::widgets::*, prelude::*};

#[component]
pub fn TaskWizardStep1() -> Element {
    let mut step = use_steps().current();
    let mut kind = use_context::<Signal<EntityKind>>();
    let mut task = use_context::<Signal<(SelectedItem, SelectedItem)>>();

    let is_quiz = kind() == EntityKind::Quiz;
    let is_survey = kind() == EntityKind::Survey;

    rsx! {
        div {
            class: "card flex-fixed bg-base-100 shadow",
            div {
                class: "card-body flex-fixed items-center gap-5",
                h2 {
                    class: "text-2xl font-bold",
                    { t!("task-wizard-step-1-title") }
                }

                div {
                    class: "flex-scrollable flex-row flex-wrap gap-5 items-center justify-center",
                    div {
                        class: format!("card cursor-pointer hover:bg-base-200 hover:shadow-lg w-50 h-50 {class}",
                            class = if is_quiz { "bg-base-200 shadow-lg" } else { "bg-base-100" }
                        ),
                        onclick: move |_| {
                            if *kind.read() == EntityKind::Quiz { return }
                            task.with_mut(|(n, _)| n.id = "".to_string());
                            kind.set(EntityKind::Quiz)
                        },
                        div {
                            class: "card-body",
                            figure {
                                class: format!("text-6xl {class}",
                                    class = if is_quiz { "text-accent" } else { "text-base-content/70" }
                                ),
                                i { class: "bi bi-mortarboard" }
                            }
                            div {
                                class: format!("card-title justify-center font-semibold {class}",
                                    class = if is_quiz { "text-accent" } else { "" }
                                ),
                                { t!("quiz-task") }
                            }
                            p {
                                class: "text-center text-base-content/70",
                                { t!("quiz-task-description") }
                            }
                        }
                    }

                    div {
                        class: format!("card cursor-pointer hover:bg-base-200 hover:shadow-lg w-50 h-50 {class}",
                            class = if is_survey { "bg-base-200 shadow-lg" } else { "bg-base-100" }
                        ),
                        onclick: move |_| {
                            if *kind.read() == EntityKind::Survey { return }
                            task.with_mut(|(n, _)| n.id = "".to_string());
                            kind.set(EntityKind::Survey)
                        },
                        div {
                            class: "card-body",
                            figure {
                                class: format!("text-6xl {class}",
                                    class = if is_survey { "text-accent" } else { "text-base-content/70" }
                                ),
                                i { class: "bi bi-incognito" }
                            }
                            div {
                                class: format!("card-title justify-center font-semibold {class}",
                                    class = if is_survey { "text-accent" } else { "" }
                                ),
                                { t!("survey-task") }
                            }
                            p {
                                class: "text-center text-base-content/70",
                                { t!("survey-task-description") }
                            }
                        }
                    }
                }

            }
            div {
                class: "card-actions justify-between mx-10 mb-5",
                button {
                    class: "btn",
                    onclick: move |_| {
                        let current_step = step();
                        if current_step == 1 {
                            use_navigator().push(Route::Tasks {});
                        } else {
                            step.set(current_step - 1);
                        }
                    },
                    { t!("previous") }
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| step.set(step() + 1),
                    { t!("next") }
                }
            }
        }
    }
}
