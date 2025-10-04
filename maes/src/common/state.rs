use crate::prelude::*;

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