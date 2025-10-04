mod error;
mod result;

pub use self::{error::*, result::*};

#[cfg(any(feature = "desktop", feature = "server"))]
pub use crate::{safe_nanoid, t};
