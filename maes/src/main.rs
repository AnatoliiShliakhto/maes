#![windows_subsystem = "windows"]
#![allow(dead_code)]
#![allow(unused_macros)]
pub mod common;
pub mod components;
pub mod elements;
pub mod pages;
pub mod services;

pub mod prelude {
    pub use super::{
        api_call,
        api_fetch,
        common::*,
        // api_call_async,
        // api_fetch_async,
        make_ctx_menu,
    };
    pub use ::dioxus::prelude::*;
    pub use ::shared::{
        common::*, form_values, models::*, payloads::*, safe_nanoid, services::*, t, utils::*,
    };
    pub use ::std::sync::{Arc, RwLock};
    pub use ::tracing::error;
}

use ::dioxus::desktop::{
    Config as LaunchBuilderConfig, LogicalPosition, LogicalSize, WindowBuilder,
};
use components::widgets::*;
use elements::*;
use pages::*;
use prelude::*;
use services::*;

fn main() {
    let app_data_path = dirs::data_dir()
        .unwrap()
        .join(env!("CARGO_PKG_NAME"))
        .canonicalize()
        .unwrap();
    let _log_guard = init_file_logger(
        env!("CARGO_PKG_NAME"),
        &app_data_path.join("logs").to_string_lossy(),
    );
    let config = ConfigService::read();
    let server_handle = server::launch_server(config.server.clone(), app_data_path.clone());

    let window = WindowBuilder::new()
        .with_resizable(true)
        .with_maximized(config.main_window.maximized)
        .with_transparent(false)
        .with_always_on_top(false)
        .with_decorations(false)
        .with_content_protection(false)
        .with_title(t!("app-title"))
        .with_window_icon(create_window_icon(include_bytes!("../assets/icon.png")))
        .with_position(LogicalPosition::new(
            config.main_window.left,
            config.main_window.top,
        ))
        .with_inner_size(LogicalSize::new(
            config.main_window.width,
            config.main_window.height,
        ))
        .with_min_inner_size(LogicalSize::new(800, 700));

    let launch_builder_config = LaunchBuilderConfig::new()
        .with_resource_directory("assets")
        .with_data_directory(app_data_path)
        .with_disable_context_menu(false)
        .with_window(window)
        .with_menu(None);

    LaunchBuilder::new()
        .with_cfg(launch_builder_config)
        .launch(|| {
            let app_state = use_app_state();

            rsx! {
                div {
                    class: "flex-fixed h-dvh w-dvw min-h-screen",
                    oncontextmenu: move |evt| {
                        if !cfg!(debug_assertions) {
                            evt.prevent_default();
                        }
                    },
                    Head {}
                    AppHeader {}

                    if app_state() == AppState::Authorized {
                        Router::<Route> {}
                    } else {
                        Login {}
                    }

                    ToastContainer { key: "toast-container" }
                    Resizer { key: "resizer" }
                }
            }
        });

    server_handle.shutdown()
}
