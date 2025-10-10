use super::*;
use crate::{components::*, prelude::*, services::*};

#[component]
pub fn SurveyStart(workspace: ReadSignal<String>, task: ReadSignal<String>) -> Element {
    let navigator = use_navigator();

    use_effect(move || {
        api_fetch!(
            GET,
            format!("/api/v1/activities/{workspace}/{task}"),
            on_success = move |body: SurveyRecord| {
                SURVEY.signal().set(body);
                CURRENT.signal().set(0);
                navigator.replace(Route::SurveyTake {});
            },
            on_error = move |e: shared::common::Error| ErrorService::show(t!(e.to_string()))
        )
    });

    rsx! { Loading {} }
}
