use super::manager::WindowKind;
use crate::{prelude::*, services::*, elements::*, components::widgets::*, reports::*};
use ::dioxus::desktop::{
    Config as LaunchBuilderConfig, LogicalPosition, LogicalSize, WindowBuilder, use_window
};

pub fn open_child_window(title: impl Into<String>, kind: WindowKind) {
    let title = title.into();
    let config = ConfigService::read();

    use_window().new_window(
        VirtualDom::new_with_props(ChildWindow, ChildWindowProps { title: title.clone(), kind }),
        LaunchBuilderConfig::new()
            .with_window(
                WindowBuilder::new()
                    .with_title(title)
                    .with_window_icon(create_window_icon(include_bytes!("../../assets/icon.png")))
                    .with_resizable(true)
                    .with_maximized(config.windows.child.maximized)
                    .with_transparent(false)
                    .with_always_on_top(false)
                    .with_decorations(false)
                    .with_content_protection(false)
                    .with_position(LogicalPosition::new(
                        config.windows.child.left,
                        config.windows.child.top,
                    ))
                    .with_inner_size(LogicalSize::new(
                        config.windows.child.width,
                        config.windows.child.height,
                    ))
                    .with_min_inner_size(LogicalSize::new(800, 700))
            )
            .with_data_directory(app_data_path())
            .with_resource_directory("assets")
            .with_disable_context_menu(true)
            .with_menu(None),
    );
}

#[component]
fn ChildWindow(title: String, kind: WindowKind) -> Element{

    let set_title_eval = format!(r#"document.title = "{title}";"#);
    use_hook(move || {
        to_owned![set_title_eval];
        document::eval(set_title_eval.as_str());
    });

    rsx! {
        Head { is_main: false }
        div {
            class: "flex-fixed h-dvh w-dvw min-h-screen print:min-h-screen-0 print:w-full print:h-full",
            oncontextmenu: move |evt| evt.prevent_default(),
            ChildHeader { title }
            match kind {
                WindowKind::WiFiInstruction => rsx! { WiFiInstruction {} },
                WindowKind::QuizTickets { task } => rsx! { QuizTickets { task } },
                WindowKind::SurveyTickets { task } => rsx! { SurveyTickets { task } },
                _ => rsx! {},
            }
        }
        ToastContainer { key: "toast-container" }
        Resizer {}
    }
}