#![allow(dead_code)]
use crate::service::env;
use ::std::{sync::OnceLock, thread::available_parallelism, time::Duration};
use ::tokio::runtime::{Builder, Runtime};
use ::tracing::info;

const DEFAULT_WORKER_THREADS: usize = 4;
pub const DEFAULT_STACK_SIZE: usize = 3 * 1024 * 1024; // 3 MiB
const MIN_STACK_SIZE: usize = 1024 * 1024; // 1 MiB
const MAX_STACK_SIZE: usize = 16 * 1024 * 1024; // 16 MiB
const THREAD_KEEP_ALIVE: Duration = Duration::from_secs(60);

static WORKER_THREADS: OnceLock<usize> = OnceLock::new();

fn get_worker_threads() -> usize {
    *WORKER_THREADS.get_or_init(|| {
        env::get_var("TOKIO_WORKER_THREADS")
            .and_then(|s| s.parse().ok())
            .filter(|&n| n > 0 && n <= 1024)
            .unwrap_or_else(|| {
                available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(DEFAULT_WORKER_THREADS)
            })
    })
}

fn validate_stack_size(stack_size: usize) -> usize {
    stack_size.clamp(MIN_STACK_SIZE, MAX_STACK_SIZE)
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub worker_threads: usize,
    pub stack_size: usize,
    pub thread_name: String,
    pub enable_io: bool,
    pub enable_time: bool,
    pub thread_keep_alive: Duration,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            worker_threads: get_worker_threads(),
            stack_size: DEFAULT_STACK_SIZE,
            thread_name: "service-worker".to_string(),
            enable_io: true,
            enable_time: true,
            thread_keep_alive: THREAD_KEEP_ALIVE,
        }
    }
}

impl RuntimeConfig {
    pub fn high_performance() -> Self {
        Self {
            worker_threads: get_worker_threads(),
            stack_size: 4 * 1024 * 1024, // 4 MiB
            thread_name: "hp-worker".to_string(),
            enable_io: true,
            enable_time: true,
            thread_keep_alive: Duration::from_secs(300), // 5 mins
        }
    }

    pub fn memory_efficient() -> Self {
        Self {
            worker_threads: (get_worker_threads() / 2).max(1),
            stack_size: 2 * 1024 * 1024, // 2 MiB
            thread_name: "mem-worker".to_string(),
            enable_io: true,
            enable_time: true,
            thread_keep_alive: Duration::from_secs(30),
        }
    }

    pub fn with_worker_threads(mut self, threads: usize) -> Self {
        self.worker_threads = threads.clamp(1, 1024);
        self
    }

    pub fn with_stack_size(mut self, size: usize) -> Self {
        self.stack_size = validate_stack_size(size);
        self
    }

    pub fn with_thread_name(mut self, name: impl Into<String>) -> Self {
        self.thread_name = name.into();
        self
    }
}

pub fn build_runtime(stack_size: usize) -> Runtime {
    build_runtime_with_config(RuntimeConfig::default().with_stack_size(stack_size))
}

pub fn build_runtime_with_config(config: RuntimeConfig) -> Runtime {
    let mut builder = Builder::new_multi_thread();
    builder
        .worker_threads(config.worker_threads)
        .thread_name(&config.thread_name)
        .thread_stack_size(config.stack_size)
        .thread_keep_alive(config.thread_keep_alive);
    if config.enable_io && config.enable_time {
        builder.enable_all();
    } else {
        if config.enable_io {
            builder.enable_io();
        }
        if config.enable_time {
            builder.enable_time();
        }
    }
    builder.build().expect("failed to create tokio runtime")
}

pub fn build_service_runtime() -> Runtime {
    let config = RuntimeConfig::default();
    info!(
        "creating runtime with {} worker threads, {} stack size",
        config.worker_threads, config.stack_size
    );
    build_runtime_with_config(config)
}

static GLOBAL_RUNTIME: OnceLock<Runtime> = OnceLock::new();

pub fn get_global_runtime() -> &'static Runtime {
    GLOBAL_RUNTIME.get_or_init(build_service_runtime)
}

#[macro_export]
macro_rules! multi_thread_runtime {
    ($app: ident) => {{
        use ::maes_util::util::runtime::{build_runtime, DEFAULT_STACK_SIZE};
        let rt = build_runtime(DEFAULT_STACK_SIZE);
        rt.block_on($app())
    }};
    ($app: ident, $config: expr) => {{
        use ::maes_util::util::runtime::build_runtime_with_config;
        let rt = build_runtime_with_config($config);
        rt.block_on($app())
    }};
}

#[macro_export]
macro_rules! service_runtime {
    (high_performance, $app: ident) => {{
        use ::maes_util::util::runtime::{build_runtime_with_config, RuntimeConfig};
        let rt = build_runtime_with_config(RuntimeConfig::high_performance());
        rt.block_on($app())
    }};
    (memory_efficient, $app: ident) => {{
        use ::maes_util::util::runtime::{build_runtime_with_config, RuntimeConfig};
        let rt = build_runtime_with_config(RuntimeConfig::memory_efficient());
        rt.block_on($app())
    }};
    (global, $app: ident) => {{
        use ::maes_util::util::runtime::get_global_runtime;
        let rt = get_global_runtime();
        rt.block_on($app())
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_runtime_creation_performance() {
        let start = Instant::now();
        let _rt = build_service_runtime();
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < 100,
            "runtime creation took too long: {duration:?}"
        );
    }

    #[test]
    fn test_worker_threads_validation() {
        let config = RuntimeConfig::default().with_worker_threads(0);
        assert_eq!(config.worker_threads, 1);

        let config = RuntimeConfig::default().with_worker_threads(2000);
        assert_eq!(config.worker_threads, 1024);
    }

    #[test]
    fn test_stack_size_validation() {
        let config = RuntimeConfig::default().with_stack_size(100);
        assert_eq!(config.stack_size, MIN_STACK_SIZE);

        let config = RuntimeConfig::default().with_stack_size(100 * 1024 * 1024);
        assert_eq!(config.stack_size, MAX_STACK_SIZE);
    }
}
