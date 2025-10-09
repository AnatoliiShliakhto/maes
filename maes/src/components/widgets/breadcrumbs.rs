use crate::prelude::*;
use ::std::collections::HashMap;

const SEGMENT_ICONS: &[(&str, &str)] = &[
    ("about", "bi bi-info-circle"),
    ("reports", "bi bi-journal-text"),
    ("tasks", "bi bi-activity"),
    ("wizard", "bi bi-magic"),
    ("workspace", "bi bi-person-workspace"),
    ("surveys", "bi bi-incognito"),
    ("quizzes", "bi bi-mortarboard"),
    ("settings", "bi bi-gear"),
    ("students", "bi bi-people"),
];

#[component]
pub fn Breadcrumbs() -> Element {
    let route = use_route::<Route>();
    let segments = extract_route_segments(&route);
    let icon_map = create_icon_map();

    rsx! {
        div { class: "breadcrumbs text-sm mx-2.5 text-accent overflow-hidden",
            ul {
                { render_home_breadcrumb() }
                { render_segment_breadcrumbs(&segments, &icon_map) }
            }
        }
    }
}

fn extract_route_segments(route: &Route) -> Vec<String> {
    route
        .to_string()
        .split('/')
        .filter(|s| !s.is_empty() && SEGMENT_ICONS.iter().any(|(key, _)| key == s))
        .map(|s| s.to_string())
        .collect()
}

fn create_icon_map() -> HashMap<&'static str, &'static str> {
    SEGMENT_ICONS.iter().copied().collect()
}

fn render_home_breadcrumb() -> Element {
    rsx! {
        li {
            Link {
                to: Route::Tasks {},
                i { class: "bi bi-house-door" }
            }
        }
    }
}

fn render_segment_breadcrumbs(segments: &[String], icon_map: &HashMap<&str, &str>) -> Element {
    rsx! {
        {segments.iter().enumerate().map(|(i, segment)| {
            let path = build_breadcrumb_path(segments, i);
            let icon_class = icon_map.get(segment.as_str()).unwrap_or(&"");

            rsx! {
                li {
                    Link {
                        to: "{path}",
                        { render_segment_icon(icon_class) }
                        { t!(segment) }
                    }
                }
            }
        })}
    }
}

fn build_breadcrumb_path(segments: &[String], index: usize) -> String {
    format!("/{}", segments[..=index].join("/"))
}

fn render_segment_icon(icon_class: &str) -> Element {
    if icon_class.is_empty() {
        rsx! {}
    } else {
        rsx! { i { class: "{icon_class}" } }
    }
}
