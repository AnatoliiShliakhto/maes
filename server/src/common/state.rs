use ::shared::common::*;
use ::std::{path::PathBuf, sync::OnceLock};
pub use ::shared::services::dispatcher::*;

static IDENT: OnceLock<String> = OnceLock::new();
static PATH: OnceLock<PathBuf> = OnceLock::new();
static DISPATCHER: OnceLock<Dispatcher> = OnceLock::new();

#[derive(Copy, Clone)]
pub struct State;

impl State {
    pub fn init(ident: impl Into<String>, path: impl Into<PathBuf>, dispatcher: Dispatcher) -> Result<()> {
        IDENT.set(ident.into()).map_err(map_log_err)?;
        PATH.set(path.into()).map_err(|_| "State init failed")?;
        DISPATCHER.set(dispatcher).map_err(|_| "State dispatcher failed")?;
        Ok(())
    }
    
    pub fn ident() -> &'static str {
        IDENT.get().unwrap().as_str()
    }
    pub fn path() -> &'static PathBuf {
        PATH.get().unwrap()
    }
    
    pub fn dispatcher() -> &'static Dispatcher {
        DISPATCHER.get().unwrap()
    }
}
