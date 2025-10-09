use super::*;
use crate::{components::*, prelude::*, services::*};

#[component]
pub fn QuizFinish() -> Element {
    let navigator = use_navigator();
    let quiz = QUIZ.signal();

    if quiz.read().task.is_empty() {
        navigator.push(Route::Home {});
        return rsx! {};
    }

    use_effect(move || {
        let quiz_guard = quiz.read();
        let questions = quiz_guard
            .questions
            .values()
            .map(|q| {
                (
                    q.id.clone(),
                    QuizActivityQuestion {
                        id: q.id.clone(),
                        category: q.category.clone(),
                        kind: q.kind,
                        name: q.name.clone(),
                        img: q.img,
                        answers: Default::default(),
                        answered: q.answered.clone(),
                    },
                )
            })
            .collect();

        api_call!(
            POST,
            "/api/v1/activities",
            QuizActivity {
                workspace: quiz_guard.workspace.clone(),
                task: quiz_guard.task.clone(),
                quiz: quiz_guard.quiz.clone(),
                duration: quiz_guard.duration,
                student: quiz_guard.student.clone(),
                questions,
            },
            on_success = move || {
                let quiz_guard = quiz.read();
                navigator.replace(Route::QuizDetails {
                    workspace: quiz_guard.workspace.clone(),
                    task: quiz_guard.task.clone(),
                    student: quiz_guard.student.clone(),
                });
            },
            on_error = move |e: shared::common::Error| ErrorService::show(t!(e.to_string()))
        )
    });

    rsx! { Loading {} }
}
