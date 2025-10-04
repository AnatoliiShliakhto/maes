#![allow(dead_code)]
use crate::common::*;
use ::tracing::level_filters::LevelFilter;
use ::tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use ::tracing_subscriber::{
    fmt::layer,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {EnvFilter, Registry},
};

#[derive(Clone)]
pub struct LoggerConfig {
    pub app_name: &'static str,
    pub log_level: LevelFilter,
    pub file_path: Option<String>,
    pub max_log_files: usize,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            app_name: env!("CARGO_PKG_NAME"),
            log_level: LevelFilter::INFO,
            file_path: None,
            max_log_files: 30,
        }
    }
}

fn create_env_filter(default_level: LevelFilter) -> EnvFilter {
    EnvFilter::builder()
        .with_default_directive(default_level.into())
        .from_env_lossy()
}

fn create_stdout_layer() -> impl tracing_subscriber::Layer<Registry> {
    layer().compact().with_target(true)
}

pub fn init_logger(config: LoggerConfig) -> Result<Option<WorkerGuard>> {
    let stdout_layer = create_stdout_layer();
    let mut layers: Vec<Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync>> =
        vec![Box::new(stdout_layer)];
    let mut guard = None;
    if let Some(file_path) = &config.file_path {
        let file_appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix(config.app_name)
            .filename_suffix("log")
            .max_log_files(config.max_log_files)
            .build(file_path)
            .map_err(map_log_err)?;
        let (non_blocking, file_guard) = tracing_appender::non_blocking(file_appender);
        let file_layer = layer()
            .compact()
            .with_ansi(false)
            .with_target(true)
            .with_writer(non_blocking);

        layers.push(Box::new(file_layer));
        guard = Some(file_guard);
    }
    let env_filter = create_env_filter(config.log_level);
    tracing_subscriber::registry()
        .with(layers)
        .with(env_filter)
        .try_init()
        .map_err(map_log_err)?;
    Ok(guard)
}

pub fn init_stdout_logger() -> Result<()> {
    init_logger(LoggerConfig::default())?;
    Ok(())
}

pub fn init_file_logger(app: &'static str, path: &str) -> Result<WorkerGuard> {
    let config = LoggerConfig {
        app_name: app,
        file_path: Some(path.to_string()),
        ..Default::default()
    };
    init_logger(config)?.ok_or_else(|| "logger guard not created".into())
}
