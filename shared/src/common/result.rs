use super::*;

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "server")]
pub trait ResultExt<T> {
    fn err_into(self) -> Result<T>;
    fn discard(self) -> Result<()>;
}

#[cfg(feature = "server")]
impl<T> ResultExt<T> for surrealdb::Result<T> {
    #[inline]
    fn err_into(self) -> Result<T> { self.map_err(Into::into) }
    
    #[inline]
    fn discard(self) -> Result<()> { self.map(|_| ()).map_err(Into::into) }
}