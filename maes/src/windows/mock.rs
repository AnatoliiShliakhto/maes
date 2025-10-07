use crate::{prelude::*, services::*, elements::*, components::widgets::*};
use ::dioxus::desktop::{
    Config as LaunchBuilderConfig, LogicalPosition, LogicalSize, WindowBuilder, use_window
};

pub fn mock_window(url: String) {
    let config = ConfigService::read();

    use_window().new_window(
        VirtualDom::new_with_props(MockWindow, MockWindowProps { url }),
        LaunchBuilderConfig::new()
            .with_window(
                WindowBuilder::new()
                    .with_title(t!("app-title"))
                    .with_window_icon(create_window_icon(include_bytes!("../../assets/icon.png")))
                    .with_resizable(true)
                    .with_maximized(false)
                    .with_transparent(false)
                    .with_always_on_top(false)
                    .with_decorations(false)
                    .with_content_protection(false)
                    .with_position(LogicalPosition::new(
                        config.windows.mock.left,
                        config.windows.mock.top,
                    ))
                    .with_inner_size(LogicalSize::new(
                        config.windows.mock.width,
                        config.windows.mock.height,
                    ))
                    .with_min_inner_size(LogicalSize::new(400, 400))
                    .with_max_inner_size(LogicalSize::new(800, 600))
            )
            .with_data_directory(app_data_path())
            .with_resource_directory("assets")
            .with_disable_context_menu(true)
            .with_menu(None),
    );
}

#[component]
fn MockWindow(url: String) -> Element{
    let url = url.clone();

    rsx! {
        Head {}
        div {
            class: "flex-fixed h-dvh w-dvw min-h-screen",
            oncontextmenu: move |evt| evt.prevent_default(),
            MockHeader {}
            iframe {
                class: "w-full h-full",
                src: "{url}",
            }
        }
        Resizer {}
    }
}