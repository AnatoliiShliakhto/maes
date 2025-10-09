use crate::{pages::*, prelude::*, services::*, components::dialogs::*};

#[component]
pub fn SurveyTree() -> Element {
    let claims = AuthService::claims();

    let survey = use_context::<Signal<Survey>>();
    let mut selected = use_context::<Signal<SurveyManagerAction>>();
    let survey_guard = survey.read();
    let node_class = if SurveyManagerAction::Survey == *selected.read() {
        "bg-base-300"
    } else {
        ""
    };

    let create_category_action =
        use_callback(move |_| {
            ToastService::info(t!("fill-form-message"));
            selected.set(SurveyManagerAction::Category("".to_string()))
        });

    let ctx_menu = make_ctx_menu!([
        (t!("create-survey-category"),
        "bi bi-folder-plus",
        create_category_action,
        false,
        true),
    ]);
    
    rsx! {
        ul {
            class: "menu flex-wrap",
            li {
                key: "{survey_guard.id}",
                div {
                    class: "font-semibold text-primary {node_class}",
                    oncontextmenu: move |evt| {
                        if claims.is_admin() { ctx_menu(evt) } else { evt.stop_propagation() }
                    },
                    onclick: move |_| selected.set(SurveyManagerAction::Survey),
                    i { class: "bi bi-incognito" }
                    "{survey_guard.name}"
                }
                ul {
                    for (category_id, _category) in survey.read().categories.iter() {
                        RenderSurveyTreeCategory {
                            key: "{category_id}",
                            category_id: "{category_id}",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderSurveyTreeCategory(category_id: ReadSignal<String>) -> Element {
    let claims = AuthService::claims();
    let mut survey = use_context::<Signal<Survey>>();
    let mut selected = use_context::<Signal<SurveyManagerAction>>();
    let survey_guard = survey.read();

    let Some(category) = survey_guard.categories.get(&*category_id.read()) else {
        return rsx! {};
    };
    let node_class = match &*selected.read() {
        SurveyManagerAction::Category(id) if id == &*category_id.read() => "bg-base-300",
        _ => "",
    };

    let delete_category_action = {
        let category_name = category.name.clone();
        let callback = use_callback(move |_| {
            api_fetch!(
                DELETE,
                format!(
                    "/api/v1/manager/surveys/{survey_id}/{category_id}",
                    survey_id = survey.read().id,
                    category_id = category_id.read()
                ),
                on_success = move |body: String| {
                    survey.with_mut(|s| {
                        s.categories.shift_remove(&body);
                    });
                    if body == *category_id.read() {
                        selected.set(SurveyManagerAction::Survey);
                    }
                }
            )
        });
        use_callback(move |_| {
            use_dialog().warning(
                t!("delete-survey-category-message", name = category_name.clone()),
                Some(callback),
            )
        })
    };

    let ctx_menu = make_ctx_menu!([(t!("delete"), "bi bi-trash", delete_category_action)]);

    let select_action =
        move |_| selected.set(SurveyManagerAction::Category(category_id.read().clone()));    
    
    rsx! {
        li {
            div {
                class: "{node_class}",
                onclick: select_action,
                oncontextmenu: move |evt| if claims.is_admin() { ctx_menu(evt) } else { evt.stop_propagation() },
                i { class: "bi bi-folder text-base-content/70" }
                "{category.name}"
            }
        }        
    }
}