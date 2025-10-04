use ::std::path::PathBuf;
use ::serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub wifi: WiFiConfig,
    pub recent: RecentConfig,
    pub main_window: WindowConfig,
    pub child_window: WindowConfig,
    pub menu_state: bool,
    pub language: String,
    pub theme: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct WiFiConfig {
    pub ssid: String,
    pub password: String,
    pub start: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct WindowConfig {
    pub top: i32,
    pub left: i32,
    pub width: i32,
    pub height: i32,
    pub maximized: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            top: 20,
            left: 100,
            width: 1200,
            height: 700,
            maximized: false,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct RecentConfig {
    pub workspace: String,
    pub login: String,
    pub export: PathBuf,
    pub import: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub ident: String,
    pub host: String,
    pub remote: bool,
}
