use crate::common::*;
use ::axum::{
    extract::{ConnectInfo, FromRequestParts},
    http::{header::USER_AGENT, request::Parts},
};
use ::shared::common::*;
use ::serde::{Deserialize, Serialize};
use ::std::net::SocketAddr;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Connection {
    pub ip: String,
    pub host: Option<String>,
    pub user_agent: Option<String>,
}

impl Connection {
    pub fn is_local(&self) -> bool {
        self.ip.starts_with("127.")
            || self.ip.starts_with("::1")
            || self.ip.starts_with("0:0:")
            || self
                .host
                .as_ref()
                .map(|s| s.as_str().starts_with("localhost"))
                .unwrap_or(false)
    }

    pub fn is_app(&self) -> bool {
        self.user_agent == IDENT.get().cloned()
    }

    pub fn checked_local(&self) -> Result<()> {
        if !self.is_local() {
            Err((StatusCode::FORBIDDEN, "forbidden").into())
        } else {
            Ok(())
        }
    }

    pub fn checked(&self) -> Result<()> {
        if !self.is_app() {
            Err((StatusCode::FORBIDDEN, "forbidden").into())
        } else {
            Ok(())
        }
    }
}

impl<S> FromRequestParts<S> for Connection
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let ip = get_client_ip(parts).unwrap_or_default();
        let host = parts
            .headers
            .get("host")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        let user_agent = parts
            .headers
            .get(USER_AGENT)
            .and_then(|v| v.to_str().ok().map(|s| s.to_string()));
        Ok(Connection {
            ip,
            host,
            user_agent,
        })
    }
}

fn get_client_ip(parts: &Parts) -> Result<String> {
    let headers = &parts.headers;
    if let Some(forwarded_for) = headers.get("x-forwarded-for")
        && let Ok(header_value) = forwarded_for.to_str()
        && let Some(first_ip) = header_value.split(',').next()
    {
        return Ok(first_ip.trim().to_string());
    }
    if let Some(real_ip) = headers.get("x-real-ip")
        && let Ok(ip) = real_ip.to_str()
    {
        return Ok(ip.to_string());
    }
    if let Some(forwarded_host) = headers.get("x-forwarded-host")
        && let Ok(host) = forwarded_host.to_str()
    {
        return Ok(host.to_string());
    }
    if let Some(connect_info) = parts.extensions.get::<ConnectInfo<SocketAddr>>() {
        return Ok(connect_info.0.ip().to_string());
    }
    Err((StatusCode::BAD_REQUEST, "bad-request").into())
}
