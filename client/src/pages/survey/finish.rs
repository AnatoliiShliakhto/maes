use super::*;
use crate::{components::*, prelude::*, services::*};

#[component]
pub fn SurveyFinish() -> Element {
    let navigator = use_navigator();
    let survey = SURVEY.signal();

    if survey.read().id.is_empty() {
        navigator.push(Route::Home {});
        return rsx! {};
    }

    use_effect(move || {
        let survey = SURVEY();
        api_call!(
            POST,
            "/api/v1/activities",
            survey,
            on_success = move || {
                navigator.replace(Route::SurveyRetry {});
            },
            on_error = move |e: shared::common::Error| ErrorService::show(t!(e.to_string()))
        )
    });

    rsx! { Loading {} }
}
