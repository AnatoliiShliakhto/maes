use crate::{prelude::*, components::{widgets::*, quiz::*, dialogs::*}, services::*};

#[derive(Clone, PartialEq)]
pub enum QuizManagerAction {
    Quiz,
    Category(String),
    Question(String, String),
}

#[component]
pub fn QuizManager(quiz_id: ReadOnlySignal<String>) -> Element {
    if !AuthService::claims().is_supervisor() { return rsx! {} }
    use_init_dialog();
    use_init_input_dialog();
    use_init_context_menu();

    use_context_provider(|| Signal::new(QuizManagerAction::Quiz));
    let mut quiz = use_context_provider(|| Signal::new(Quiz::default()));

    use_effect(move || {
        api_fetch!(
            GET,
            format!("/api/v1/manager/quizzes/{id}", id = quiz_id.read()),
            on_success = move |body: Quiz| quiz.set(body),
        )
    });

    rsx! {
        SplitPanel {
            key: "{quiz_id}",
            left_title: t!("quiz-navigator"),
            left: rsx! {
                div {
                    class: "flex-scrollable",
                    if !quiz.read().id.is_empty() {
                        QuizTree {}
                    }
                }
            },
            //right_title: t!("quiz-editor"),
            right: rsx! {
                div {
                    class: "flex-fixed",
                    if !quiz.read().id.is_empty() {
                        QuizEditor {}
                    }
                }
            }
        }
        DialogContainer { key: "quiz-manager-dialog" }
        InputDialogContainer { key: "quiz-manager-dialog" }
        ContextMenuContainer { key: "quiz-manager-ctx-menu" }
    }
}