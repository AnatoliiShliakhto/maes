#![allow(dead_code)]
pub mod common;
mod handlers;
mod middleware;
mod repositories;
mod services;

use crate::{common::*, services::*};
use ::axum_server::Handle;
use ::shared::{common::*, models::*, services::*, utils::*};
use ::std::net::SocketAddr;
use ::tokio::{fs, net::TcpListener};
use ::tracing::error;

pub fn launch_server(config: ServerConfig, dispatcher: Dispatcher) -> Handle {
    let handle = Handle::new();
    let handle_cloned = handle.clone();
    std::thread::spawn(move || {
        build_runtime_with_config(RuntimeConfig::high_performance())
            .block_on(main(config, handle_cloned, dispatcher))
            .map_err(|e| error!("{e}"))
            .unwrap();
    });
    handle
}

async fn main(config: ServerConfig, handle: Handle, dispatcher: Dispatcher) -> Result<()> {
    let data_path = dirs::data_dir()
        .unwrap()
        .join("maes");
    let ws_path = data_path.join("workspaces");
    if !ws_path.exists() {
        _ = fs::create_dir_all(ws_path).await.map_err(|e| error!("Failed to create workspace directory: {e}"));
    }
    
    State::init(&config.ident, &data_path, dispatcher)?;
    ExchangeService::init();
    TextSimilarityService::init().await?;
    
    let router = router::init_router(data_path, &config);
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
