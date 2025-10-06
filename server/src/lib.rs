#![allow(dead_code)]
pub mod common;
mod handlers;
mod middleware;
mod repositories;
mod services;

use crate::{common::*, services::*};
use ::axum_server::Handle;
use ::shared::{common::*, models::*, services::*, utils::*};
use ::std::{net::SocketAddr, path::PathBuf};
use ::tokio::net::TcpListener;
use ::tracing::error;

pub fn launch_server(config: ServerConfig, path: PathBuf) -> Handle {
    let handle = Handle::new();
    let handle_cloned = handle.clone();
    std::thread::spawn(move || {
        build_runtime_with_config(RuntimeConfig::high_performance())
            .block_on(main(config, path, handle_cloned))
            .map_err(|e| error!("{e}"))
            .unwrap();
    });
    handle
}

async fn main(config: ServerConfig, path: PathBuf, handle: Handle) -> Result<()> {
    Store::init(&path).await.map_err(map_log_err)?;
    ImageService::init(&path).await.map_err(map_log_err)?;
    init_state(&config.ident)
        .await
        .map_err(map_log_err)?;
    let router = router::init_router(path, &config);
    let (_scheme, _host, port) = parse_scheme_host_port(&config.host).map_err(map_log_err)?;
    let server_address = format!("0.0.0.0:{port}")
        .parse::<SocketAddr>()
        .map_err(map_log_err)?;

    TcpListener::bind(server_address)
        .await
        .map_err(map_log_err)?;

    axum_server::bind(server_address)
        .handle(handle)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .map_err(map_log_err)
}
