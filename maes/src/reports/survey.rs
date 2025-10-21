use crate::{components::widgets::*, prelude::*};

#[derive(Default, Clone, PartialEq)]
struct SurveyReportState {
    pub extended: bool,
}

#[component]
pub fn SurveyReport(entity: ReadSignal<String>) -> Element {
    let mut survey_rec = use_context_provider(|| Signal::new(SurveyRecord::default()));
    let survey_rec_guard = survey_rec.read();
    let mut state = use_context_provider(|| Signal::new(SurveyReportState { extended: true }));

    use_effect(move || {
        api_fetch!(
            GET,
            format!(
                "/api/v1/entities/payload/{kind}/{id}",
                kind = EntityKind::SurveyRecord,
                id = entity.read()
            ),
            on_success = move |body: SurveyRecord| survey_rec.set(body)
        );
    });

    if survey_rec_guard.id.is_empty() || survey_rec_guard.total == 0 {
        return rsx! {};
    }

    rsx! {
        div {
            class: "flex shrink-0 w-full min-h-0 print:hidden p-1",
            ul {
                class: "menu menu-horizontal p-0 m-0 text-base-content flex-nowrap",
                li {
                    button {
                        class: "hover:text-info",
                        onclick: move |event: MouseEvent| {
                            event.prevent_default();
                            event.stop_propagation();
                            document::eval("window.print()");
                        },
                        i { class: "bi bi-printer" }
                        { t!("print") }
                    }
                }
                div { class: "divider divider-horizontal m-1 w-1" }
                div {
                    class: "tooltip tooltip-bottom",
                    "data-tip": t!("extended"),
                    li {
                        button {
                            class: if state.read().extended { "bg-secondary/30 text-secondary" } else { "" },
                            onclick: move |_| state.with_mut(|s| s.extended = !s.extended),
                            i { class: "bi bi-bar-chart-steps" }
                        }
                    }
                }
            }
        }

        div {
            class: "flex flex-1 flex-col print-area overflow-auto px-5 items-center",
            "data-theme": "lofi",
            div {
                class: "flex flex-col w-full items-center gap-0.25 py-5",
                div {
                    class: "text-lg font-semibold",
                    "{survey_rec_guard.name}"
                }
                div { "{survey_rec_guard.path}" }
                div {
                    class: "flex w-full justify-end",
                    { t!("date-stamp", date = survey_rec_guard.metadata.created_at()) }
                    " - "
                    { t!("date-stamp", date = survey_rec_guard.metadata.updated_at()) }
                }
            }

            for category in survey_rec_guard.categories.values() {
                div {
                    class: "flex flex-col p-5 items-center break-inside-avoid",
                    div { class: "text-lg font-semibold", "{category.name}" }
                    if category.answers.is_empty() {
                        RenderQuestionsCategory { category: category.clone() }
                    } else if category.questions.is_empty() {
                        RenderAnswersCategory { category: category.clone() }
                    } else {
                        RenderMultiplyCategory { category: category.clone() }
                    }
                }
            }
            div {
                class: "flex flex-col pb-5 gap-5 w-full",
                div {
                    class: "flex flex-nowrap",
                    { t!("survey-footer", total = survey_rec_guard.total) }
                }
                div {
                    class: "flex flex-nowrap",
                    span { { t!("supervisor-sign") } }
                }
            }
        }
    }
}

#[component]
fn RenderMultiplyCategory(category: SurveyRecordCategory) -> Element {
    let state = use_context::<Signal<SurveyReportState>>();
    let survey_rec = use_context::<Signal<SurveyRecord>>();
    let survey_rec_guard = survey_rec.read();
    let total = survey_rec_guard.total;
    let questions_total = category.questions.len();

    let answers = category
        .answers
        .values()
        .map(|a| a.name.clone())
        .collect::<Vec<_>>();
    let mut answers_data = vec![];
    for i in 0..answers.len() {
        let summary: usize = category.results.get_col(i).iter().map(|&v| *v).sum();
        answers_data.push((summary * 100) / (questions_total * total));
    }
    let answers_series = vec![ChartSeries {
        name: category.name,
        data: answers_data,
    }];

    let mut series = vec![];
    for (i, answer) in answers.iter().enumerate() {
        let data = category
            .results
            .get_col(i)
            .iter()
            .map(|&v| v * 100 / total)
            .collect::<Vec<_>>();
        series.push(ChartSeries {
            name: answer.clone(),
            data,
        });
    }
    let categories = category
        .questions
        .values()
        .map(|q| q.name.clone())
        .collect::<Vec<_>>();

    rsx! {
        BarChart { series: answers_series, categories: answers, distributed: true }
        div {
            class: format!("flex w-full {class}", class = if state.read().extended { "" } else { "hidden" }),
            StackedBarChart { series, categories, distributed: false }
        }
    }
}

#[component]
fn RenderQuestionsCategory(category: SurveyRecordCategory) -> Element {
    let survey_rec = use_context::<Signal<SurveyRecord>>();
    let survey_rec_guard = survey_rec.read();
    let total = survey_rec_guard.total;
    let data = category
        .results
        .get_col(0)
        .iter()
        .map(|&v| v * 100 / total)
        .collect::<Vec<_>>();
    let series = vec![ChartSeries {
        name: category.name,
        data,
    }];
    let categories = category
        .questions
        .values()
        .map(|q| q.name.clone())
        .collect::<Vec<_>>();

    rsx! {
        BarChart { series, categories, distributed: false }
    }
}

#[component]
fn RenderAnswersCategory(category: SurveyRecordCategory) -> Element {
    let survey_rec = use_context::<Signal<SurveyRecord>>();
    let survey_rec_guard = survey_rec.read();
    let total = survey_rec_guard.total;
    let data = category
        .results
        .get_row(0)
        .iter()
        .map(|&v| v * 100 / total)
        .collect::<Vec<_>>();
    let series = vec![ChartSeries {
        name: category.name,
        data,
    }];
    let categories = category
        .answers
        .values()
        .map(|a| a.name.clone())
        .collect::<Vec<_>>();

    rsx! {
        BarChart { series, categories, distributed: true }
    }
}
