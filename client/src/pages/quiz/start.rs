use super::*;
use crate::{components::*, prelude::*, services::*};

#[component]
pub fn QuizStart(
    workspace: ReadSignal<String>,
    task: ReadSignal<String>,
    student: ReadSignal<String>,
) -> Element {
    let navigator = use_navigator();

    use_effect(move || {
        api_fetch!(
            GET,
            format!("/api/v1/activities/{workspace}/{task}/{student}"),
            on_success = move |body: QuizActivity| {
                TIMER.signal().set(body.duration);
                QUIZ.signal().set(body);
                navigator.replace(Route::QuizTake {});
            },
            on_error = move |e: shared::common::Error| ErrorService::show(t!(e.to_string()))
        )
    });

    rsx! { Loading {} }
}
