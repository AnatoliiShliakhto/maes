use crate::{services::*, prelude::*};
use ::std::{path::PathBuf, sync::LazyLock};
use ::shared::services::dispatcher::*;

static DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    dirs::data_dir()
        .unwrap()
        .join("maes")
    // dirs::data_dir()
    //     .unwrap()
    //     .join(env!("CARGO_PKG_NAME"))
    //     .canonicalize()
    //     .unwrap()
});

pub fn app_data_path() -> PathBuf {
    DATA_PATH.clone()
}

static LOCALHOST: LazyLock<String> = LazyLock::new(|| {
    let (_scheme, _host, port) = parse_scheme_host_port(&ConfigService::read().server.host).unwrap_or(("".to_string(), "".to_string(), 4583));
    format!("http://localhost:{port}")
});

pub fn localhost() -> String {
    LOCALHOST.clone()
}

static APP_STATE: GlobalSignal<AppState> = Signal::global(|| AppState::Started);

pub fn use_app_state() -> Signal<AppState> {
    APP_STATE.signal()
}

#[derive(Copy, Clone, PartialEq)]
pub enum AppState {
    Started,
    Running,
    Authorized,
}

static DISPATCHER: LazyLock<Dispatcher> = LazyLock::new(Dispatcher::new);

pub fn dispatcher() -> &'static Dispatcher {
    &DISPATCHER
}