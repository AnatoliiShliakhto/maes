use ::serde::Serialize;
use crate::prelude::*;

#[derive(Clone, PartialEq, Serialize)]
pub struct ChartSeries {
    pub name: String,
    pub data: Vec<usize>,
}

#[derive(Clone, PartialEq, Props, Serialize)]
pub struct BarChartProps {
    pub series: Vec<ChartSeries>,
    pub categories: Vec<String>,
    pub distributed: bool,
}

#[component]
pub fn BarChart(props: BarChartProps) -> Element {
    let id = safe_nanoid!();
    let payload_json = serde_json::to_string(&props)?;
    let payload_js_literal = serde_json::to_string(&payload_json)?;

    let script = format!(r#"
        (function(){{
            const payload = JSON.parse({payload_js_literal});
            function handler(){{
                if (typeof window.barChart === 'function') {{
                    window.barChart('{id}', payload);
                }}
            }}
            if (typeof window.barChart === 'function') {{ handler(); return; }}
            window.addEventListener('charts-ready', handler, {{ once: true }});
        }})();
    "#);

    use_effect(move || { document::eval(&script); });

    rsx! {
        div {
            key: "{id}",
            id: "{id}"
        }
    }
}

#[component]
pub fn StackedBarChart(props: BarChartProps) -> Element {
    let id = safe_nanoid!();
    let payload_json = serde_json::to_string(&props)?;
    let payload_js_literal = serde_json::to_string(&payload_json)?;

    let script = format!(r#"
        (function(){{
            const payload = JSON.parse({payload_js_literal});
            function handler(){{
                if (typeof window.stackedBarChart === 'function') {{
                    window.stackedBarChart('{id}', payload);
                }}
            }}
            if (typeof window.stackedBarChart === 'function') {{ handler(); return; }}
            window.addEventListener('charts-ready', handler, {{ once: true }});
        }})();
    "#);

    use_effect(move || { document::eval(&script); });

    rsx! {
        div {
            key: "{id}",
            id: "{id}"
        }
    }
}