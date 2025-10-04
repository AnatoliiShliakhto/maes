use super::result::*;
use ::std::borrow::Cow;
#[cfg(feature = "server")]
use ::tracing::error;
#[cfg(feature = "server")]
use axum::{
    response::{IntoResponse, Response},
};

#[cfg(feature = "server")]
pub use axum::http::StatusCode;

pub enum Error {
    #[cfg(feature = "server")]
    Server(StatusCode, Cow<'static, str>),
    Common(Cow<'static, str>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "server")]
            Error::Server(_, message) |
            Error::Common(message) => write!(f, "{message}"),
        }
    }
}

#[cfg(feature = "server")]
impl From<(u16, String)> for Error {
    fn from(value: (u16, String)) -> Self {
        Error::Server(
            StatusCode::from_u16(value.0).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Cow::Owned(value.1),
        )
    }
}

#[cfg(feature = "server")]
impl From<(u16, &'static str)> for Error {
    fn from(value: (u16, &'static str)) -> Self {
        Error::Server(
            StatusCode::from_u16(value.0).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Cow::Borrowed(value.1),
        )
    }
}

#[cfg(feature = "server")]
impl From<(StatusCode, String)> for Error {
    fn from(value: (StatusCode, String)) -> Self {
        Error::Server(value.0, Cow::Owned(value.1))
    }
}

#[cfg(feature = "server")]
impl From<(StatusCode, &'static str)> for Error {
    fn from(value: (StatusCode, &'static str)) -> Self {
        Error::Server(value.0, Cow::Borrowed(value.1))
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Common(Cow::Owned(value))
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Error::Common(Cow::Borrowed(value))
    }
}

#[inline]
pub fn map_ok<T>(_value: T) -> Result<()> {
    Ok(())
}

// #[inline]
// pub fn map_err_ok<T, E: std::fmt::Display>(value: std::result::Result<T, E>) -> Result<T> {
//     value.map_err(map_err)
// }

#[inline]
pub fn map_err(e: impl Into<String>) -> Error {
    Error::Common(Cow::Owned(e.into()))
}

#[cfg(feature = "server")]
#[inline]
pub fn map_log_err(e: impl std::fmt::Display) -> Error {
    error!("{e}");
    (StatusCode::INTERNAL_SERVER_ERROR, "internal-server-error").into()
}

#[cfg(feature = "server")]
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Server(status, message) => (status, message).into_response(),
            Error::Common(message) => (StatusCode::INTERNAL_SERVER_ERROR, message).into_response(),
        }
    }
}
