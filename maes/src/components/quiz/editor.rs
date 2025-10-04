use super::{editor_category::*, editor_question::*, editor_quiz::*};
use crate::{pages::*, prelude::*};

#[component]
pub fn QuizEditor() -> Element {
    let quiz = use_context::<Signal<Quiz>>();
    let selected = use_context::<Signal<QuizManagerAction>>();

    rsx! {
        match &*selected.read() {
            QuizManagerAction::Quiz => rsx! {
                QuizEditorQuiz {
                    key: "{quiz.read().id}",
                }
            },
            QuizManagerAction::Category(category_id) => rsx! {
                QuizEditorCategory {
                    key: "editor-{category_id}",
                    category_id: "{category_id}",
                }
            },
            QuizManagerAction::Question(category_id, question_id) => rsx! {
                QuizEditorQuestion {
                    key: "editor-{category_id}-{question_id}",
                    category_id: "{category_id}",
                    question_id: "{question_id}",
                }
            },
        }
    }
}
