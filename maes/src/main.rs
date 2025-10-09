#![windows_subsystem = "windows"]
#![allow(dead_code)]
#![allow(unused_macros)]
pub mod common;
pub mod components;
pub mod elements;
pub mod pages;
pub mod services;
mod windows;

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
        form_values, models::*, payloads::*, safe_nanoid, services::*, t, utils::*,
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
    let app_data_path = app_data_path();
    let _log_guard = init_file_logger(
        env!("CARGO_PKG_NAME"),
        &app_data_path.join("logs").to_string_lossy(),
    );
    let config = ConfigService::read();
    let server_handle = server::launch_server(config.server.clone(), app_data_path.clone());

    let window = WindowBuilder::new()
        .with_resizable(true)
        .with_maximized(config.windows.main.maximized)
        .with_transparent(false)
        .with_always_on_top(false)
        .with_decorations(false)
        .with_content_protection(false)
        .with_title(t!("app-title"))
        .with_window_icon(create_window_icon(include_bytes!("../assets/icon.png")))
        .with_position(LogicalPosition::new(
            config.windows.main.left,
            config.windows.main.top,
        ))
        .with_inner_size(LogicalSize::new(
            config.windows.main.width,
            config.windows.main.height,
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
            let mut app_state = use_app_state();

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

                    match app_state() {
                        AppState::Started => {
                            let mut counter = use_signal(|| 0);
                            use_effect(move || {
                                let _counter_guard = counter.read();
                                api_call!(
                                    GET,
                                    "/health",
                                    on_success = move || app_state.set(AppState::Running),
                                    on_error = move |e: shared::common::Error| {
                                        if counter() < 5 {
                                            counter.set(counter() + 1)
                                        } else {
                                            error!("server check heath failed: {e}");
                                            ToastService::error(t!(e.to_string()))
                                        }
                                    },
                                )
                            });

                            rsx! {
                                div {
                                    class: "flex-fixed items-center justify-center",
                                    div {
                                        class: "inline-flex items-center gap-6",
                                        span {
                                            class: "loading loading-bars size-12"
                                        }
                                        span {
                                            class: "text-2xl font-semibold",
                                            { t!("loading-resources") }
                                        }
                                    }
                                }
                            }
                        },
                        AppState::Running => rsx! { Login {} },
                        AppState::Authorized => rsx! { Router::<Route> {} },
                    }

                    ToastContainer { key: "toast-container" }
                    Resizer { key: "resizer" }
                }
            }
        });

    server_handle.shutdown()
}
