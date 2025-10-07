use ::std::path::PathBuf;
use ::serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub wifi: WiFiConfig,
    pub windows: WindowsConfig,
    pub recent: RecentConfig,
    pub language: String,
    pub theme: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct WindowsConfig {
    pub main: WindowConfig,
    pub child: WindowConfig,
    pub mock: WindowConfig,
}

impl Default for WindowsConfig {
    fn default() -> Self {
        Self {
            main: WindowConfig::default(),
            child: WindowConfig::default(),
            mock: WindowConfig {
                top: 50,
                left: 50,
                width: 400,
                height: 600,
                maximized: false,
            },
        }
    }
}


#[derive(Clone, Deserialize, Serialize)]
pub struct WiFiConfig {
    pub ssid: String,
    pub password: String,
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
    pub images: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub ident: String,
    pub host: String,
    pub remote: bool,
}
