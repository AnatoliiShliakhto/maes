#![allow(dead_code)]
use ::std::{env, fs, path::Path, str::FromStr};

pub fn get_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

pub fn get_static_var(key: &str) -> Option<&'static str> {
    env::var(key)
        .ok()
        .map(|value| Box::leak(value.into_boxed_str()) as &'static str)
}

pub fn read_secret_or_env(env_var: &str, secret_path: &str) -> Option<String> {
    #[allow(clippy::collapsible_if)]
    if Path::new(secret_path).exists() {
        if let Ok(content) = fs::read_to_string(secret_path) {
            return Some(content.trim().to_string());
        }
    }
    get_var(env_var)
}

pub fn parse_env_var<T: FromStr>(key: &str) -> Option<T> {
    env::var(key).ok().and_then(|value| value.parse().ok())
}

pub fn parse_env_var_or_default<T: FromStr + Clone>(key: &str, default: T) -> T {
    env::var(key)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

pub fn parse_bool_env(key: &str) -> Option<bool> {
    env::var(key)
        .ok()
        .map(|value| matches!(value.to_lowercase().as_str(), "true" | "1" | "yes" | "on"))
}

pub fn parse_bool_env_or_default(key: &str, default: bool) -> bool {
    parse_bool_env(key).unwrap_or(default)
}
