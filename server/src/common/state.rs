use ::shared::common::*;
use ::std::sync::OnceLock;

pub static IDENT: OnceLock<String> = OnceLock::new();

pub async fn init_state(ident: impl Into<String>) -> Result<()> {
    IDENT.set(ident.into()).map_err(map_log_err)?;
    Ok(())
}
